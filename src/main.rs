use crate::infra::mutation::Mutation;
use crate::infra::tracer::init_tracing_subscriber;
use actix_web::dev::Service;
use actix_web::http::header::HeaderName;
use actix_web::http::header::HeaderValue;
use actix_web::http::Error;
use actix_web::middleware;
use actix_web::web;
use actix_web::HttpMessage;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use infra::list_post::list;
use listenfd::ListenFd;
use log::info;
use migration::Migrator;
use migration::MigratorTrait;
use opentelemetry::global;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::env;
use std::time::Duration;
use tracing_actix_web::RequestId;
use tracing_actix_web::TracingLogger;

mod infra;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tracing::instrument]
async fn foo() {
    tracing::info!(
        monotonic_counter.foo = 1_u64,
        key_1 = "bar",
        key_2 = 10,
        "handle foo",
    );

    tracing::info!(histogram.baz = 10, "histogram example",);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("RUST_BACKTRACE", "1");

    let _guard = init_tracing_subscriber();

    let _tracer = global::tracer("app_fmm");

    // get env vars
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    // .sqlx_logging_level(log::LevelFilter::Off);

    // tracer.in_span("main-operation", |cx| {
    //     let span = cx.span();
    //     span.set_attribute(KeyValue::new("my-attribute", "my-value"));
    //     span.add_event(
    //         "Main span event".to_string(),
    //         vec![KeyValue::new("foo", "1")],
    //     );
    //     tracer.in_span("child-operation...", |cx| {
    //         let span = cx.span();
    //         span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
    //     });
    // });
    let conn = Database::connect(opt).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();
    let state = AppState { conn };
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(TracingLogger::default())
            // set the request id in the `x-request-id` response header
            .wrap_fn(|req, srv| {
                let request_id = req.extensions().get::<RequestId>().copied();
                let res = srv.call(req);
                async move {
                    let mut res = res.await?;
                    if let Some(request_id) = request_id {
                        res.headers_mut().insert(
                            HeaderName::from_static("x-request-id"),
                            HeaderValue::from_str(&request_id.to_string()).unwrap(),
                        );
                    }
                    Ok(res)
                }
            })
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(not_found))
            .configure(init)
    });
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(&server_url)?,
    };
    info!("Starting server at {server_url}");
    server.run().await?;
    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub text: String,
}

#[tracing::instrument(
    name = "Create::Post",
    skip(data),
    fields(client_id, request_id),
    ret,
    err
)]
#[post("/")]
async fn create(
    data: web::Data<AppState>,
    payload: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, Error> {
    let span = tracing::Span::current();
    span.record("client_id", "client_id:1234");
    // span.record("request_id", request_id.to_string());

    let conn = &data.conn;

    let form = payload.into_inner();

    let result = Mutation::create_post(conn, form)
        .await
        .map_err(|e| {
            // debug!("{:?}", e);
            tracing::error! {
                %e,
                "Error creating post"
            };
        })
        .expect("Error creating post");

    Ok(HttpResponse::Created().json(result))
}

async fn not_found() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("<h1>404</h1><p>Not Found</p>"))
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(list);
    cfg.service(hello);
}

use crate::infra::mutation::Mutation;
use actix_web::http::Error;
use actix_web::middleware;
use actix_web::web;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use infra::list_post::list;
use listenfd::ListenFd;
use log::debug;
use log::info;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::ConnectOptions;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::TryIntoModel;
use std::env;
use std::time::Duration;

mod infra;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "trace");
    std::env::set_var("RUST_BACKTRACE", "1");

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_test_writer()
    //     .init();

    tracing_subscriber::fmt().with_test_writer().init();

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
        .max_lifetime(Duration::from_secs(8));
    // .sqlx_logging(false);
    // .sqlx_logging_level(log::LevelFilter::Off);

    let conn = Database::connect(opt).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
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

#[post("/")]
async fn create(
    data: web::Data<AppState>,
    payload: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    let form = payload.into_inner();

    let result = Mutation::create_post(conn, form)
        .await
        .map_err(|e| {
            debug!("{:?}", e);
        })
        .expect("Error creating post")
        .try_into_model()
        .unwrap();

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

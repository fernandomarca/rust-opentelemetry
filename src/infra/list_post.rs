use super::post::Model;
use crate::infra::query::Query;
use crate::AppState;
use actix_web::get;
use actix_web::http::Error;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    per_page: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostListPresenter {
    itens: Vec<Model>,
    num_pages: u64,
}

#[get("/list")]
async fn list(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;

    // get params
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let posts_per_page = params.per_page.unwrap_or(10);

    let (posts, num_pages) = Query::find_posts_in_page(conn, page, posts_per_page)
        .await
        .expect("Cannot find posts in page");

    Ok(HttpResponse::Ok().json(PostListPresenter {
        itens: posts,
        num_pages,
    }))
}

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use crate::db::get_posts;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! This is riton!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/posts")]
async fn posts() -> Result<HttpResponse> {
    let posts = get_posts();
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin","*")
        .json(posts)
    )
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(posts);
}
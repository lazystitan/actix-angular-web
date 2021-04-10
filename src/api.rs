extern crate derive_more;

use actix_web::{get, HttpResponse, post, Responder, Result, web, error};
use crate::db::{get_posts, get_post};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "api error: code => {}, message => {}", code, message)]
struct ApiError {
    code: i32,
    message: String
}

impl error::ResponseError for ApiError {}

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
async fn posts() -> Result<HttpResponse, ApiError> {
    let posts_result = get_posts();
    match posts_result {
        Ok(posts) => Ok(HttpResponse::Ok()
            .header("Access-Control-Allow-Origin", "*")
            .json(posts)),
        Err(error) => Err(ApiError {
            code: 1,
            message: error.to_string()
        })
    }
}

#[get("/post/{post_id}")]
async fn post(web::Path(post_id) : web::Path<i32>) -> Result<HttpResponse, ApiError> {
    let post = get_post(post_id);
    match post {
        Ok(post) => Ok(HttpResponse::Ok()
            .header("Access-Control-Allow-Origin","*")
            .json(post)
        ),
        Err(error) => Err(ApiError {
            code: 0,
            message: error.to_string()
        })
    }

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(posts)
        .service(post);
}
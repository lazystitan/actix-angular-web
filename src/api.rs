extern crate derive_more;

use actix_web::{get, HttpResponse, post, Responder, Result, web, HttpRequest};
use crate::db::{get_posts, get_post};
use crate::error::ApiError;
use std::path::Path;
use std::fs::File;
use std::io::Read;

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
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let index_path = Path::new("./app/dist/app/index.html");
    let mut file = match File::open(&index_path) {
        Ok(file) => file,
        Err(why) => panic!("index not found! {} {}", index_path.display(), why)
    };

    let mut index_content = String::new();
    match file.read_to_string(&mut index_content) {
        Ok(_) => Ok(HttpResponse::Ok().body(index_content)),
        Err(why) => panic!("index read failed! {}", why)
    }

}

#[get("/{filename:.+\\.(css|js|icon)}")]
async fn static_file(req : HttpRequest) -> Result<HttpResponse> {
    let mut full_path = "./app/dist/app/".to_owned();
    let file_name = req.match_info().query("filename");
    full_path.push_str(file_name);
    let path = Path::new(&full_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("{} not found! {} {}", file_name, path.display(), why)
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(HttpResponse::Ok().body(content)),
        Err(why) => panic!("index read failed! {} {}", file_name, why)
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
            web::scope("/apis")
                .service(posts)
                .service(post)
        )
        .service(index)
        .service(static_file);
}
extern crate derive_more;

use crate::error::ApiError;
use crate::{db, DbPool};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Result};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/posts")]
async fn posts(pool: web::Data<DbPool>) -> Result<HttpResponse, ApiError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let posts_result = web::block(move || db::get_posts(&conn))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });
    match posts_result {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(error) => Ok(error)
    }
}

#[get("/post/{post_id}")]
async fn post(
    pool: web::Data<DbPool>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let post = web::block(move || db::get_post(post_id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });
    match post {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(error) => Ok(error)
    }
}
#[get("/")]
async fn index() -> Result<HttpResponse> {
    let index_path = Path::new("./app/dist/app/index.html");
    let mut file = match File::open(&index_path) {
        Ok(file) => file,
        Err(why) => panic!("index not found! {} {}", index_path.display(), why),
    };

    let mut index_content = String::new();
    match file.read_to_string(&mut index_content) {
        Ok(_) => Ok(HttpResponse::Ok().body(index_content)),
        Err(why) => panic!("index read failed! {}", why),
    }
}

#[get("/{filename:.+\\.(css|js|icon)}")]
async fn static_file(req: HttpRequest) -> Result<HttpResponse> {
    let mut full_path = "./app/dist/app/".to_owned();
    let file_name = req.match_info().query("filename");
    full_path.push_str(file_name);
    let path = Path::new(&full_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("{} not found! {} {}", file_name, path.display(), why),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(HttpResponse::Ok().body(content)),
        Err(why) => panic!("index read failed! {} {}", file_name, why),
    }
}

#[get("/panic/{flag}")]
async fn panic_sim(web::Path(flag): web::Path<bool>) -> Result<HttpResponse, ApiError> {
    if flag {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("not panic"))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/apis").service(posts).service(post))
        .service(index)
        .service(static_file)
        .service(panic_sim);
}

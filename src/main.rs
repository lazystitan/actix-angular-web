#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use diesel::prelude::*;
use diesel::PgConnection;
use std::env;
use models::*;

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

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_posts() -> Vec<Post> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .load::<Post>(&connection)
        .expect("Error loading posts");
    return results;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(posts)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod db;
pub mod api;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use db::*;
use api::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
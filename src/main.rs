#[macro_use]
extern crate diesel;
extern crate derive_more;

pub mod schema;
pub mod models;
pub mod db;
pub mod api;
pub mod error;

use actix_web::{App, HttpServer};

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
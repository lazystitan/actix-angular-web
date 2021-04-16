#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;
pub mod error;
pub mod logger;
pub mod models;
pub mod schema;

use actix_web::{dev, http, middleware, App, HttpServer};

use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use api::config;
use diesel::pg::Pg;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::Logger::init_config();
    dotenv::dotenv().ok();
    let db_pool = db::build_db_conn_pool();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[macro_use]
extern crate diesel;

pub mod api;
pub mod db;
pub mod error;
pub mod logger;
pub mod models;
pub mod schema;

use actix_web::{middleware, App, HttpServer};
use actix_web::middleware::Logger;
use api::config;
use diesel::{r2d2, PgConnection, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
            .wrap(error::get_error_handlers())
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

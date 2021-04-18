#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

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
use std::{io, fs};
use rustls::internal::pemfile;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::Logger::init_config();
    println!("logger init success!");
    dotenv::dotenv().ok();
    println!("env init success!");
    let db_pool = db::build_db_conn_pool();
    println!("db pool init success!");
    let mut ssl_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let cert_file = &mut io::BufReader::new(fs::File::open("ssl/5503875_www.ritonelion.com.pem").unwrap());
    let key_file = &mut io::BufReader::new(fs::File::open("ssl/5503875_www.ritonelion.com.key").unwrap());
    let cert_chain = pemfile::certs(cert_file).unwrap();
    let mut keys = pemfile::rsa_private_keys(key_file).unwrap();
    println!("ssl pool init success!");

    ssl_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            // .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(error::get_error_handlers())
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .bind_rustls("0.0.0.0:8083", ssl_config)?
    .run()
    .await
}

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
use actix_session::{CookieSession, Session};
use api::config;
use std::{io, fs, env};
use rustls::internal::pemfile;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //get stage
    let args:Vec<_> = env::args().collect();
    let mut stage = "dev";
    if args.len() >= 2 && args[1] == "--prod" {
        stage = "prod";
    }

    //init logger
    logger::Logger::init_config();
    println!("logger init success!");

    //init env
    dotenv::dotenv().ok();
    println!("env init success!");

    //init db
    let db_pool = db::DataService::new(stage);
    println!("db pool init success!");

    //init ssl
    let mut ssl_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());
    let cert_file = &mut io::BufReader::new(fs::File::open("ssl/5503875_www.ritonelion.com.pem").unwrap());
    let key_file = &mut io::BufReader::new(fs::File::open("ssl/5503875_www.ritonelion.com.key").unwrap());
    let cert_chain = pemfile::certs(cert_file).unwrap();
    let mut keys = pemfile::rsa_private_keys(key_file).unwrap();
    ssl_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    println!("ssl init success!");

    //init server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            // .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .wrap(error::get_error_handlers())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .bind_rustls("0.0.0.0:8083", ssl_config)?
    .run()
    .await
}

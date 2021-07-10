#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

pub mod apis;
pub mod db;
pub mod error;
pub mod models;
pub mod schema;
pub mod ssl;
pub mod util;

use actix_cors::Cors;
use actix_web::middleware::{Condition, Logger};
use actix_web::{App, HttpServer};
use std::{env};

pub fn running_stage() -> &'static str {
    let args: Vec<_> = env::args().collect();
    let mut stage = "dev";
    if args.len() >= 2 && args[1] == "--prod" {
        stage = "prod";
    }
    stage
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //get stage
    let stage = running_stage();

    //init logger
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap_or_else(|e| {
        panic!("init logger file failed: {:?}", e);
    });
    info!("logger init success!");

    //init env
    dotenv::dotenv().ok();
    info!("env init success!");

    //init db
    let db_pool = db::DataService::new(stage);
    info!("db pool init success!");

    //init ssl
    let pem_file_path = "ssl/5503875_www.ritonelion.com.pem";
    let private_path = "ssl/5503875_www.ritonelion.com.key";
    let mut ssl_builder = ssl::SslConfigBuiler::new();
    ssl_builder.set_ssl_files(pem_file_path, private_path);
    info!("ssl init success!");

    //init server
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Condition::new(stage == "dev", Cors::permissive()))
            .configure(apis::gen_config(stage))
    })
    .bind("0.0.0.0:8080")?
    .bind_rustls("0.0.0.0:8083", ssl_builder.build())?
    .run()
    .await
}

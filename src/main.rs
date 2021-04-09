use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, Result, web};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct Hero {
    id: i32,
    name: String,
}

#[get("/heroes")]
async fn heroes() -> Result<HttpResponse> {
    let heros = vec![Hero { id: 11, name: String::from("Dr Nice") },
                     Hero { id: 12, name: String::from("Narco") },
                     Hero { id: 13, name: String::from("Bombasto") },
                     Hero { id: 14, name: String::from("Celeritas") },
                     Hero { id: 15, name: String::from("Magneta") },
                     Hero { id: 16, name: String::from("RubberMan") },
                     Hero { id: 17, name: String::from("Dynama") },
                     Hero { id: 18, name: String::from("Dr IQ") },
                     Hero { id: 19, name: String::from("Magma") },
                     Hero { id: 20, name: String::from("Tornado") }];

    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .json(heros))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(heroes)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
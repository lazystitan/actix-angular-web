use actix_web::{get, post, HttpResponse,  Responder, Result, web};
use crate::error::ApiError;
use actix_session::Session;

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/panic/{flag}")]
pub async fn panic_sim(web::Path(flag): web::Path<bool>) -> Result<HttpResponse, ApiError> {
    if flag {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("not panic"))
    }
}

#[get("/session/add")]
pub async fn add_counter(session: Session) -> Result<HttpResponse> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }


    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}",
        session.get::<i32>("counter")?.unwrap()
    )))
}





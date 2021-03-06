use actix_web::{get, post, HttpResponse,  Responder, Result as AResult, web};
use crate::error::CustomError;
use actix_session::Session;

#[post("/echo")]
pub async fn echo_handle(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/panic/{flag}")]
pub async fn panic_sim(web::Path(flag): web::Path<bool>) -> AResult<HttpResponse, CustomError> {
    if flag {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("not panic"))
    }
}

#[get("/session/add")]
pub async fn add_counter(session: Session) -> AResult<HttpResponse> {
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

#[get("/error_test/{id}")]
pub async fn error_test(web::Path(id): web::Path<i32>) -> AResult<HttpResponse, CustomError> {
    match id {
        0 => {
            Err(CustomError::BadClientData("sadfsadf".to_string()))
        }
        _ => {
            Err(CustomError::InternalError("sadfsadf".to_string()))
        }
    }
}





use actix_web::{error, http};
use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
struct ApiError {
    code: i32,
    message: String
}

#[derive(Debug, Serialize)]
pub enum CustomError {
    InternalError(String),
    BadClientData(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let r;
        match self {
            CustomError::InternalError(m) => {
                r = ApiError {
                    code: 1,
                    message: m.clone()
                }
            }
            CustomError::BadClientData(m) => {
                r = ApiError {
                    code: 2,
                    message: m.clone()
                }
            }
        }
        write!(f, "{}", serde_json::to_string(&r).unwrap())
    }
}

impl error::ResponseError for CustomError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            CustomError::InternalError(_) => { http::StatusCode::INTERNAL_SERVER_ERROR }
            CustomError::BadClientData(_) => { http::StatusCode::BAD_REQUEST }
        }
    }
}

// fn render_500<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<errhandlers::ErrorHandlerResponse<B>> {
//     res.response_mut().headers_mut().insert(
//         http::header::CONTENT_TYPE,
//         http::HeaderValue::from_static("Error"),
//     );
//     Ok(errhandlers::ErrorHandlerResponse::Response(res))
// }
//
// pub fn get_error_handlers<B:'static>() -> errhandlers::ErrorHandlers<B>{
//     errhandlers::ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
// }
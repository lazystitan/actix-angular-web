use actix_web::{error, http};
use std::fmt;
use std::fmt::Formatter;
use serde::Serialize;
use serde_json;

#[derive(Debug, Serialize)]
struct ApiErrorDisplay {
    code: i32,
    message: String
}

#[derive(Debug, Serialize)]
pub enum ApiError {
    InternalError(String),
    BadClientData(String),
    StaticNotFound,
    StaticInternalError
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let r;
        match self {
            ApiError::InternalError(m) => {
                r = ApiErrorDisplay {
                    code: 1,
                    message: m.clone()
                }
            }
            ApiError::BadClientData(m) => {
                r = ApiErrorDisplay {
                    code: 2,
                    message: m.clone()
                }
            }
            ApiError::StaticNotFound => {
                return write!(f, "Not Found")
            }
            ApiError::StaticInternalError => {
                return write!(f, "Internal Error")
            }
        }
        write!(f, "{}", serde_json::to_string(&r).unwrap())
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            ApiError::InternalError(_) => { http::StatusCode::INTERNAL_SERVER_ERROR }
            ApiError::StaticInternalError => {http::StatusCode::INTERNAL_SERVER_ERROR}
            ApiError::BadClientData(_) => { http::StatusCode::BAD_REQUEST }
            ApiError::StaticNotFound => { http::StatusCode::NOT_FOUND }
        }
    }
}
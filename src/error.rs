use actix_web::error;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "api error: code => {}, message => {}", code, message)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

impl error::ResponseError for ApiError {}

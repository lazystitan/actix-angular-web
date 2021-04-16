use actix_web::{error, dev, http};
use derive_more::{Display, Error};
use actix_web::middleware::errhandlers;

#[derive(Debug, Display, Error)]
#[display(fmt = "api error: code => {}, message => {}", code, message)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

impl error::ResponseError for ApiError {}

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> actix_web::Result<errhandlers::ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(errhandlers::ErrorHandlerResponse::Response(res))
}

pub fn get_error_handlers<B:'static>() -> errhandlers::ErrorHandlers<B>{
    errhandlers::ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500)
}
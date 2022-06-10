use actix_web::{HttpRequest, web};
use std::sync;
use crate::db;
use crate::error::ApiError;

pub async fn do_after_validation<F, I>(req: HttpRequest, data_service: sync::Arc<db::DataService>, f: F) -> Result<I, ApiError>
    where
        F: FnOnce() -> Result<I, diesel::result::Error> + Send + 'static,
        I: Send + 'static
{
    let mut authorize_pass = false;
    if let Some(token_req) = req.headers().get("Authorization") {
        let token = token_req.to_str().unwrap().to_string();
        let data_service_arc = data_service.clone();
        if let Ok(_) = web::block(move || data_service_arc.validate_token(&token)).await.map_err(
            |e| {
                error!("{:?}", e);
                ApiError::InternalError("Internal error".to_string())
            }
        ) {
            authorize_pass = true;
        }
    }

    if authorize_pass {
        return match web::block(f).await {
            Ok(Ok(v)) => { Ok(v) }
            Ok(Err(error)) => {
                error!("{:?}", error);
                Err(ApiError::InternalError(error.to_string()))
            },
            Err(error) => {
                error!("{:?}", error);
                Err(ApiError::InternalError(error.to_string()))
            },
        }
    } else {
        error!("Validation failed for {:?}", req.headers().get("Authorization"));
        Err(ApiError::BadClientData("Validation failed".to_string()))
    }

}
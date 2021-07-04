use crate::db;
use crate::error::CustomError;
use crate::models::PostInsert;
use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Result as AResult};

#[get("/posts")]
pub async fn posts(data_service: web::Data<db::DataService>) -> AResult<HttpResponse, CustomError> {
    info!("get posts");
    let posts_result = web::block(move || data_service.get_posts())
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            CustomError::InternalError("Internal error".to_string())
        });
    match posts_result {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(error) => Err(error),
    }
}

#[get("/post/{post_id}")]
pub async fn post(
    data_service: web::Data<db::DataService>,
    web::Path(post_id): web::Path<i32>,
) -> AResult<HttpResponse, CustomError> {
    let post = web::block(move || data_service.get_post(post_id))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            CustomError::InternalError("Internal error".to_string())
        });
    match post {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(error) => Err(error),
    }
}

#[delete("/post/{post_id}")]
pub async fn delete_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, CustomError> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(_) = data_service.validate_token(token.to_str().unwrap()) {
            return match web::block(move || data_service.delete_post(post_id)).await.map_err(
                |e| {
                    eprintln!("{:?}", e);
                    CustomError::InternalError("Internal error".to_string())
                }
            ) {
                Ok(_) => Ok(HttpResponse::Ok().body("{\"code\":0}")),
                Err(e) => Err(e)
            }
        }
    }
    Err(CustomError::BadClientData("Validation failed".to_string()))
}

#[post("/post")]
pub async fn add_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    form: web::Json<PostInsert>,
) -> AResult<HttpResponse, CustomError> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(_) = data_service.validate_token(token.to_str().unwrap()) {
            let post_insert = form.0;
            return match data_service.add_post(post_insert) {
                Ok(_) => Ok(HttpResponse::Ok().body("{\"code\":0}")),
                Err(e) => Err(CustomError::InternalError(e.to_string()))
            }
        }
    }
    Err(CustomError::BadClientData("Validation failed".to_string()))
}

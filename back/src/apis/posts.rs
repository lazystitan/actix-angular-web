use crate::db;
use crate::error::CustomError;
use crate::models::PostInsert;
use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Result as AResult};

#[get("/posts")]
pub async fn posts(data_service: web::Data<db::DataService>) -> AResult<HttpResponse, CustomError> {
    info!("get all posts");
    let posts_result = web::block(move || data_service.get_posts())
        .await
        .map_err(|e| {
            error!("{:?}", e);
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
    info!("get post {}", post_id);
    let post = web::block(move || data_service.get_post(post_id))
        .await
        .map_err(|e| {
            error!("{:?}", e);
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
) -> AResult<HttpResponse, CustomError> {
    info!("try delete post {}", post_id);
    if let Some(token_req) = req.headers().get("Authorization") {
        let token = token_req.to_str().unwrap().to_string();
        let data_service_arc = (*data_service).clone();
        if let Ok(_) = web::block(move || data_service_arc.validate_token(&token)).await.map_err(
            |e| {
                error!("{:?}", e);
                CustomError::InternalError("Internal error".to_string())
            }
        ) {
            let data_service_arc = (*data_service).clone();
            return match web::block(move || data_service_arc.delete_post(post_id)).await.map_err(
                |e| {
                    error!("{:?}", e);
                    CustomError::InternalError("Internal error".to_string())
                }
            ) {
                Ok(_) => {
                    info!("delete post {}", post_id);
                    Ok(HttpResponse::Ok().body("{\"code\":0}"))
                },
                Err(e) => Err(e)
            };
        }
    }
    error!("Validation failed for {:?}", req.headers().get("Authorization"));
    Err(CustomError::BadClientData("Validation failed".to_string()))
}

#[post("/post")]
pub async fn add_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    form: web::Json<PostInsert>,
) -> AResult<HttpResponse, CustomError> {
    info!("add post");
    if let Some(token_req) = req.headers().get("Authorization") {
        let token = token_req.to_str().unwrap().to_string();
        let data_service_arc = (*data_service).clone();
        return match web::block(move || data_service_arc.validate_token(&token)).await.map_err(
            |e| {
                error!("{:?}", e);
                CustomError::InternalError("Internal error".to_string())
            }
        ) {
            Ok(_) => {
                let post_insert = form.0;
                let data_service_arc = (*data_service).clone();
                match web::block(move || data_service_arc.add_post(post_insert)).await.map_err(
                    |e| {
                        error!("{:?}", e);
                        CustomError::InternalError("Internal error".to_string())
                    }
                ) {
                    Ok(_) => {
                        info!("post added");
                        Ok(HttpResponse::Ok().body("{\"code\":0}"))
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => Err(e)
        }
    }
    error!("Validation failed for {:?}", req.headers().get("Authorization"));
    Err(CustomError::BadClientData("Validation failed".to_string()))
}

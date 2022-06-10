use actix_web::{delete, get, HttpRequest, HttpResponse, patch, post, Result as AResult, web};

use crate::db;
use crate::error::ApiError;
use crate::models::PostInsert;
use crate::util::do_after_validation;

#[get("/posts")]
pub async fn posts(data_service: web::Data<db::DataService>) -> AResult<HttpResponse, ApiError> {
    info!("get all posts");
    let posts_result = web::block(move || data_service.get_posts()).await;

    match posts_result {
        Ok(Ok(posts)) => Ok(HttpResponse::Ok().json(posts)),
        Ok(Err(error)) => {
            error!("{:?}", error);
            Err(ApiError::InternalError(error.to_string()))
        },
        Err(error) => {
            error!("{:?}", error);
            Err(ApiError::InternalError(error.to_string()))
        },
    }
}

#[get("/post/{post_id}")]
pub async fn post(
    data_service: web::Data<db::DataService>,
    info: web::Path<i32>,
) -> AResult<HttpResponse, ApiError> {
    let post_id = info.into_inner();

    info!("get post {}", post_id);

    let post = web::block(move || data_service.get_post(post_id)).await;

    match post {
        Ok(Ok(post)) => Ok(HttpResponse::Ok().json(post)),
        Ok(Err(error)) => {
            error!("{:?}", error);
            Err(ApiError::InternalError(error.to_string()))
        },
        Err(error) => {
            error!("{:?}", error);
            Err(ApiError::InternalError(error.to_string()))
        },
    }
}

#[delete("/post/{post_id}")]
pub async fn delete_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    info: web::Path<i32>,
) -> AResult<HttpResponse, ApiError> {
    let post_id = info.into_inner();
    info!("try delete post {}", post_id);
    let data_service_arc = (*data_service).clone();

    return match do_after_validation(req, data_service_arc.clone(), move || data_service_arc.delete_post(post_id)).await {
        Ok(_) => {
            info!("delete post {}", post_id);
            Ok(HttpResponse::Ok().body("{\"code\":0}"))
        }
        Err(e) => Err(e)
    };
}

#[post("/post")]
pub async fn add_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    data: web::Json<PostInsert>,
) -> AResult<HttpResponse, ApiError> {
    info!("add post");
    let data_service_arc = (*data_service).clone();
    let post_insert = data.0;
    return match do_after_validation(req, data_service_arc.clone(), move || data_service_arc.add_post(post_insert)).await {
        Ok(_) => {
            info!("post added");
            let o_post = web::block(move || data_service.get_latest_add_post()).await;
            match o_post {
                Ok(Ok(p)) => Ok(HttpResponse::Ok().body(format!("{{\"code\":0,\"id\":{}}}", p.id))),
                Ok(Err(error)) => {
                    error!("{:?}", error);
                    Err(ApiError::InternalError(error.to_string()))
                },
                Err(error) => {
                    error!("{:?}", error);
                    Err(ApiError::InternalError(error.to_string()))
                },
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    };
}

#[patch("/post/{post_id}")]
pub async fn update_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    data: web::Json<PostInsert>,
    info : web::Path<i32>,
) -> AResult<HttpResponse, ApiError> {
    let post_id = info.into_inner();
    info!("update post");
    let data_service_arc = (*data_service).clone();
    let post_insert = data.0;
    return match do_after_validation(req, data_service_arc.clone(), move || data_service_arc.update_post(post_id, post_insert)).await {
        Ok(_) => {
            info!("post updated");
            let o_post = web::block(move || data_service.get_post(post_id))
                .await
                .map_err(|e| {
                    error!("{:?}", e);
                    ApiError::InternalError("Internal error".to_string())
                });
            match o_post {
                Ok(Ok(p)) => Ok(HttpResponse::Ok().body(format!("{{\"code\":0,\"id\":{}}}", p.id))),
                Ok(Err(error)) => Err(ApiError::InternalError(error.to_string())),
                Err(error) => Err(error),
            }
        }
        Err(e) => {
            error!("{:?}", e);
            Err(e)
        }
    };
}

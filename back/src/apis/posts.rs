use crate::db;
use crate::error::CustomError;
use crate::models::PostInsert;
use actix_web::{get, post, delete, web, HttpRequest, HttpResponse, Result as AResult};
use crate::util::do_after_validation;

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
    let data_service_arc = (*data_service).clone();
    return match do_after_validation(req, data_service_arc.clone(), move || data_service_arc.delete_post(post_id)).await {
        Ok(_) => {
            info!("delete post {}", post_id);
            Ok(HttpResponse::Ok().body("{\"code\":0}"))
        },
        Err(e) => Err(e)
    };

}

#[post("/post")]
pub async fn add_post(
    req: HttpRequest,
    data_service: web::Data<db::DataService>,
    data: web::Json<PostInsert>,
) -> AResult<HttpResponse, CustomError> {
    info!("add post");
    let data_service_arc = (*data_service).clone();
    let post_insert = data.0;
    return match do_after_validation(req, data_service_arc.clone(), move || data_service_arc.add_post(post_insert)).await {
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

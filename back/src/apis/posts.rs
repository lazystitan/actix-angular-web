use actix_web::{get, post, web, HttpResponse, HttpRequest, Result as AResult};
use crate::db;
use crate::error::ApiError;
use crate::models::PostInsert;

#[get("/posts")]
pub async fn posts(data_service: web::Data<db::DataService>) -> AResult<HttpResponse, ApiError> {
    info!("get posts");
    let posts_result = web::block(move || data_service.get_posts()).await.map_err(|e| {
        eprintln!("{:?}", e);
        HttpResponse::InternalServerError().finish()
    });
    match posts_result {
        Ok(posts) => Ok(HttpResponse::Ok().json(posts)),
        Err(error) => Ok(error),
    }
}

#[get("/post/{post_id}")]
pub async fn post(
    data_service: web::Data<db::DataService>,
    web::Path(post_id): web::Path<i32>,
) -> AResult<HttpResponse, ApiError> {
    let post = web::block(move || data_service.get_post(post_id))
        .await
        .map_err(|e| {
            eprintln!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        });
    match post {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(error) => Ok(error),
    }
}

#[post("/post")]
pub async fn add_post(req: HttpRequest,data_service: web::Data<db::DataService>, form: web::Json<PostInsert>) -> AResult<HttpResponse> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(_) = data_service.validate_token(token.to_str().unwrap()) {
            let post_insert = form.0;
            let res = data_service.add_post(post_insert);
            if let Ok(_) = res {
                return Ok(HttpResponse::Ok().body("{\"code\":0}"))
            }
        }

    }
    Ok(HttpResponse::Ok().body("{\"code\":1}"))
}

#[post("/post_form")]
pub async fn add_post_form(data_service: web::Data<db::DataService>, form: web::Form<PostInsert>) -> AResult<HttpResponse> {
    let post_insert = form.0;
    let res = data_service.add_post(post_insert);
    match res {
        Ok(_) => Ok(HttpResponse::Ok().body("ok")),
        Err(_) => Ok(HttpResponse::Ok().body("nok"))
    }
}
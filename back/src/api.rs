extern crate derive_more;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use actix_web::{get, HttpRequest, HttpResponse, post, Responder, Result, web};

use crate::db;
use crate::error::ApiError;
use actix_session::Session;
use crate::models::{PostInsert, LoginFormData};

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/posts")]
async fn posts(data_service: web::Data<db::DataService>) -> Result<HttpResponse, ApiError> {
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
async fn post(
    data_service: web::Data<db::DataService>,
    web::Path(post_id): web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
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
async fn add_post(data_service: web::Data<db::DataService>, form: web::Json<PostInsert>) -> Result<HttpResponse> {
    let post_insert = form.0;
    let res = data_service.add_post(post_insert);
    match res {
        Ok(_) => Ok(HttpResponse::Ok().body("{\"code\":0}")),
        Err(_) => Ok(HttpResponse::Ok().body("{\"code\":1}"))
    }
}

#[post("/post_form")]
async fn add_post_form(data_service: web::Data<db::DataService>, form: web::Form<PostInsert>) -> Result<HttpResponse> {
    let post_insert = form.0;
    let res = data_service.add_post(post_insert);
    match res {
        Ok(_) => Ok(HttpResponse::Ok().body("ok")),
        Err(_) => Ok(HttpResponse::Ok().body("nok"))
    }
}

#[get("/{route:.*}")]
async fn index() -> Result<HttpResponse> {
    let index_path = Path::new("../app/dist/app/index.html");
    let mut file = match File::open(&index_path) {
        Ok(file) => file,
        Err(why) => panic!("index not found! {} {}", index_path.display(), why),
    };

    let mut index_content = String::new();
    match file.read_to_string(&mut index_content) {
        Ok(_) => Ok(HttpResponse::Ok().body(index_content)),
        Err(why) => panic!("index read failed! {}", why),
    }
}

//the static files has been built
#[get("/{filename:.+\\.(css|js|icon)}")]
async fn static_file(req: HttpRequest) -> Result<HttpResponse> {
    let mut full_path = "../app/dist/app/".to_owned();
    let file_name = req.match_info().query("filename");
    full_path.push_str(file_name);
    let path = Path::new(&full_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("{} not found! {} {}", file_name, path.display(), why),
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(HttpResponse::Ok().body(content)),
        Err(why) => panic!("index read failed! {} {}", file_name, why),
    }
}

#[get("/panic/{flag}")]
async fn panic_sim(web::Path(flag): web::Path<bool>) -> Result<HttpResponse, ApiError> {
    if flag {
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().body("not panic"))
    }
}

#[get("/session/add")]
async fn add_counter(session : Session) -> Result<HttpResponse> {
    if let Some(count) = session.get::<i32>("counter")? {
        session.set("counter", count + 1)?;
    } else {
        session.set("counter", 1)?;
    }


    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}",
        session.get::<i32>("counter")?.unwrap()
    )))
}

const TOKEN: &str = "b36519073a8f8f2dfc11100975f46123";

#[post("/login")]
async fn login(_form: web::Form<LoginFormData>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body(TOKEN.clone()))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/apis")
        .service(posts)
        .service(post)
        .service(login)
        .service(add_counter)
        .service(add_post)
    )
    .service(static_file)
    .service(index)
    .service(echo)
    .route("/hello", web::get().to(manual_hello))
    .service(panic_sim);
}

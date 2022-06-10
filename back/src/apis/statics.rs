use actix_web::{get, HttpResponse, Result, HttpRequest};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use crate::error::ApiError;

#[get("/{route:.*}")]
pub async fn index() -> Result<HttpResponse, ApiError> {
    let index_path = Path::new("../app/dist/app/index.html");
    let mut file = match File::open(&index_path) {
        Ok(file) => file,
        Err(_why) => return Err(ApiError::StaticNotFound),
    };

    let mut index_content = String::new();
    match file.read_to_string(&mut index_content) {
        Ok(_) => Ok(HttpResponse::Ok().body(index_content)),
        Err(_why) => Err(ApiError::StaticInternalError),
    }
}

//the static files has been built
#[get("/{filename:.*\\..*}")]
pub async fn static_file(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let mut full_path = "../app/dist/app/".to_owned();
    let file_name = req.match_info().query("filename");
    full_path.push_str(file_name);
    let path = Path::new(&full_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_why) => return Err(ApiError::StaticNotFound),
    };

    let mut content = Vec::new();
    match file.read_to_end(&mut content) {
        Ok(_) => Ok(HttpResponse::Ok().body(content)),
        Err(_why) => Err(ApiError::StaticInternalError),
    }
}
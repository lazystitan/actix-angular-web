use actix_web::{get, HttpResponse, Result, HttpRequest};
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[get("/{route:.*}")]
pub async fn index() -> Result<HttpResponse> {
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
pub async fn static_file(req: HttpRequest) -> Result<HttpResponse> {
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
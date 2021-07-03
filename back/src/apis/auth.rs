use actix_web::{post, web, HttpResponse, Result as AResult};
use crate::db;
use crate::models::LoginFormData;

#[post("/login")]
pub async fn login(data_service: web::Data<db::DataService>, login_info: web::Json<LoginFormData>) -> AResult<HttpResponse> {
    let username = &login_info.username;

    use sha3::{Sha3_512, Digest};
    let password = login_info.password.as_bytes();
    let mut hasher = Sha3_512::new();
    hasher.update(password);
    hasher.update("riton_elion");
    let hashed_password = format!("{:x}", hasher.finalize());

    if username == "admin" && hashed_password == "b10279a4e0e52eaa152a19d168f0bb0327c644ff02be31cc12621882108b93d7316bab7abc18befc8af622028d66dddfc58dc6182bcfc0fba807b6b0602efca5" {
        match data_service.gen_token() {
            Ok(token) => {
                Ok(HttpResponse::Ok().body(format!(
                    "{{\"code\":0,\"token\":\"{}\"}}", token
                )))
            }
            Err(e) => {
                Ok(HttpResponse::Ok().body(format!(
                    "{{\"code\":1,\"message\":\"{:?}\"}}", e
                )))
            }
        }
    } else {
        Ok(HttpResponse::Ok().body(
            "{\"code\":1,\"message\": \"wrong username or passoword\"}"
        ))
    }
}
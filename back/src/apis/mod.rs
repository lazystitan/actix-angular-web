use actix_web::web;
use actix_web::web::ServiceConfig;

mod auth;
mod posts;
mod scrathces;
mod statics;

pub fn config_prod(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/apis")
            .service(posts::posts)
            .service(posts::post)
            .service(auth::login)
            .service(posts::add_post)
            .service(posts::delete_post)
    )
    .service(statics::static_file)
    .service(statics::index);
}

pub fn config_dev(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scrathces")
            .service(scrathces::add_counter)
            .service(scrathces::panic_sim)
            .service(scrathces::error_test)
            .service(scrathces::echo_handle)
            .route("/hello", web::get().to(scrathces::manual_hello))
    );
    config_prod(cfg);
}

pub fn gen_config(stage: &str) -> fn(&mut ServiceConfig) {
    match stage == "prod" {
        true => {
            config_prod
        }
        false => {
            config_dev
        }
    }
}



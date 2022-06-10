use actix_web::web;
use actix_web::web::ServiceConfig;

mod auth;
mod posts;
mod scratches;
mod statics;

pub fn config_prod(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/apis")
            .service(posts::posts)
            .service(posts::post)
            .service(auth::login)
            .service(posts::add_post)
            .service(posts::delete_post)
            .service(posts::update_post)
    )
        .service(statics::static_file)
        .service(statics::index);
}

pub fn config_dev(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scratches")
            .service(scratches::add_counter)
            .service(scratches::panic_sim)
            .service(scratches::error_test)
            .service(scratches::echo_handle)
            .route("/hello", web::get().to(scratches::manual_hello))
    );
    config_prod(cfg);
}

pub fn gen_config(stage: &str) -> fn(&mut ServiceConfig) {
    match stage == "prod" {
        true => { config_prod }
        false => { config_dev }
    }
}



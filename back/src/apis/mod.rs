use actix_web::web;

mod auth;
mod posts;
mod scrathces;
mod statics;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/apis")
            .service(posts::posts)
            .service(posts::post)
            .service(auth::login)
            .service(posts::add_post)
            .service(scrathces::add_counter),
    )
    .service(scrathces::echo_handle)
    .route("/hello", web::get().to(scrathces::manual_hello))
    .service(scrathces::panic_sim)
    .service(scrathces::error_test)
    .service(statics::static_file)
    .service(statics::index);
}



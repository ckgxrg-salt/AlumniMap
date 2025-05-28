use actix_web::web;

use crate::handlers;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::ping)
            .service(handlers::base)
            .service(handlers::get_uni_name)
            .service(handlers::universities)
            .service(handlers::profiles)
            .service(handlers::search_university),
    )
    .service(handlers::png)
    .service(handlers::index);
}

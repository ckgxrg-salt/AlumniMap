use actix_web::web;

use crate::handlers;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::ping)
            .service(handlers::universities),
    );
}

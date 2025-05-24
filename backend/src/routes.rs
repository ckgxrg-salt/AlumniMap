use actix_web::web;

use crate::handlers;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::ping)
            .service(handlers::universities)
            .service(handlers::profiles),
    )
    .service(handlers::png)
    .service(actix_files::Files::new("/", "frontend/dist").index_file("index.html"));
}

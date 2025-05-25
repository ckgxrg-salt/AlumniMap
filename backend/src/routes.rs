use actix_web::web;

use crate::handlers;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::ping)
            .service(handlers::get_uni_name)
            .service(handlers::universities)
            .service(handlers::profiles),
    )
    .service(handlers::png)
    .service(actix_files::Files::new("/", get_dist_dir()).index_file("index.html"));
}

fn get_dist_dir() -> String {
    std::env::var("DIST_DIR").unwrap_or(String::from("frontend/dist"))
}

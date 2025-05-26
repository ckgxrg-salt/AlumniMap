use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::path::PathBuf;

use crate::server::AppState;
use entity::{profile, university};

#[get("/static/{filename:.*}")]
pub async fn png(req: HttpRequest, state: web::Data<AppState>) -> actix_web::Result<NamedFile> {
    let mut path: PathBuf = req.match_info().query("filename").parse().unwrap();
    if path
        .extension()
        .is_none_or(|ext| ext != "png" && ext != "svg")
    {
        return Err(actix_web::error::ErrorNotFound(
            "This site serves only images",
        ));
    }
    path = PathBuf::from(state.assets_root.clone()).join(path);
    Ok(NamedFile::open(path)?)
}

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().body("Up and running")
}

#[get("/universities")]
pub async fn universities(state: web::Data<AppState>) -> HttpResponse {
    let list = university::Entity::find().all(&state.db).await;
    match list {
        Ok(result) => HttpResponse::Ok().json(serde_json::to_string(&result).unwrap_or_default()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/base")]
pub async fn base(state: web::Data<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::to_string(&state.base).unwrap_or_default())
}

#[get("/universities/{uni_id}")]
pub async fn get_uni_name(state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let list = university::Entity::find_by_id(path.into_inner())
        .one(&state.db)
        .await;
    match list {
        Ok(Some(result)) => HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(result.title),
        Ok(None) => HttpResponse::NotFound().body("No such university"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/profiles/{uni_id}")]
pub async fn profiles(state: web::Data<AppState>, path: web::Path<i32>) -> HttpResponse {
    let list = profile::Entity::find()
        .filter(profile::Column::UniversityId.eq(path.into_inner()))
        .all(&state.db)
        .await;
    match list {
        Ok(result) => HttpResponse::Ok().json(serde_json::to_string(&result).unwrap_or_default()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

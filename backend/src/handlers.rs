use actix_web::{get, web, HttpResponse};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::server::AppState;
use entity::{profile, university};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json("Up and running")
}

#[get("/universities")]
pub async fn universities(state: web::Data<AppState>) -> HttpResponse {
    let list = university::Entity::find().all(&state.db).await;
    match list {
        Ok(result) => HttpResponse::Ok().json(serde_json::to_string(&result).unwrap_or_default()),
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

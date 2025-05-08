#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::models::{people, universities};

async fn load_uni(ctx: &AppContext, id: i32) -> Result<universities::Model> {
    let result = universities::Entity::find_by_id(id).one(&ctx.db).await?;
    result.ok_or_else(|| Error::NotFound)
}

pub async fn by_uni_id(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    let uni = load_uni(&ctx, id).await?;
    let people = uni.find_related(people::Entity).all(&ctx.db).await?;
    format::json(people)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/people")
        .add("/by_uni_id/{id}", get(by_uni_id))
}

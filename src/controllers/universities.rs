#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::models::_entities::universities;

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let list = universities::Entity::find().all(&ctx.db).await?;
    format::json(list)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/universities/")
        .add("/", get(list))
}

use alumnimap::app::App;
use alumnimap::models::universities;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    let wcu = universities::ActiveModel {
        name: Set("Wild Chicken University".to_string()),
        icon: Set("Not Yet".to_string()),
        lon: Set(114.0),
        lat: Set(514.0),
        ..Default::default()
    };
    wcu.insert(&ctx.db).await.unwrap();

    let res = universities::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);

    Ok(())
}

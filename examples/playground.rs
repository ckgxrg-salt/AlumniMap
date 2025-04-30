use alumnimap::app::App;
use alumnimap::models::people;
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

    let goaty_goat = people::ActiveModel {
        id_num: Set(221_901),
        cn_name: Set("山羊".to_string()),
        en_name: Set(Some("Goaty Goat".to_string())),
        class_of: Set(2022),
        major: Set(Some("Goatmatics".to_string())),
        profile: Set(Some("Who asks you".to_string())),
        ..Default::default()
    };
    goaty_goat.insert(&ctx.db).await.unwrap();
    Ok(())
}

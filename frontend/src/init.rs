//! Initialise the app

use egui::{FontData, FontDefinitions};

use crate::widgets::map::WorldMap;

pub fn init_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "notosans-sc".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../assets/NotoSansSC.ttf"
        ))),
    );
    fonts.font_data.insert(
        "notosans-nf".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../assets/NotoSansNF.ttf"
        ))),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "notosans-sc".to_owned());
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "notosans-nf".to_owned());

    ctx.set_fonts(fonts);
}

pub fn create_world_map() -> WorldMap {
    WorldMap::new(
        "TODO: Use static assets".to_string(),
        entity::university::Model {
            id: 0,
            title: "Nope".to_string(),
            icon: "Nope".to_string(),
            colour: "#ff1144".to_string(),
            longitude: 23.5,
            latitude: 0.0,
        },
    )
}

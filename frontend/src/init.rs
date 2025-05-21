//! Initialise the app

use egui::Color32;
use egui::{FontData, FontDefinitions, FontFamily};

use crate::widgets::map::{Point, WorldMap};

pub fn init_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        std::sync::Arc::new(FontData::from_static(include_bytes!(
            "../assets/NotoSansSC.ttf"
        ))),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    ctx.set_fonts(fonts);
}

pub fn create_world_map() -> WorldMap {
    WorldMap::new(
        "TODO: Use static assets".to_string(),
        Point::new(0.0, 0.0, "Nope".to_string(), Color32::YELLOW),
    )
    .add_destination(Point::new(
        24.0,
        40.0,
        "Wile Chicken University".to_string(),
        Color32::RED,
    ))
    .add_destination(Point::new(
        -20.0,
        -29.0,
        "Wild Chicken University".to_string(),
        Color32::BLUE,
    ))
}

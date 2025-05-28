//! Initialise the app

use egui::{FontData, FontDefinitions};

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

pub fn set_theme(ctx: &egui::Context) {
    catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE);
    ctx.set_theme(egui::Theme::Dark);
}

//! Each individual's profile card

use crate::app::APP_URL;
use egui::Vec2;
use entity::profile;

/// Moves a value out from the struct
fn extract_string(from: Option<&String>) -> &str {
    if let Some(str) = from {
        str
    } else {
        ""
    }
}
/// Calls egui to draw everything to the screen
pub fn render(model: &profile::Model, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let image = egui::Image::new(format!("{}static/avatars/{}", *APP_URL, model.avatar))
            .fit_to_exact_size(Vec2::new(75.0, 75.0));
        ui.add(image);
        ui.separator();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(&model.name_primary);
                ui.label(extract_string(model.name_supplementary.as_ref()));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(format!("{}", model.class_of));
                });
            });
            ui.horizontal(|ui| {
                ui.label(extract_string(model.bio.as_ref()));
                ui.add_space(100.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(extract_string(model.major.as_ref()));
                });
            });
            ui.separator();
            render_contacts(model, ui);
        });
    });
    ui.separator();
}

/// Draws the collapsible contacts section
fn render_contacts(model: &profile::Model, ui: &mut egui::Ui) {
    egui::CollapsingHeader::new("󰛋".to_string())
        .id_salt(model.id)
        .show(ui, |ui| {
            egui::Grid::new(model.id).show(ui, |ui| {
                if let Some(text) = &model.wechat {
                    ui.label(format!("󰘑 {text}"));
                }
                if let Some(text) = &model.qq {
                    ui.label(format!("󰘅 {text}"));
                }
                ui.end_row();
                if let Some(text) = &model.email {
                    ui.label(format!("󰇮 {text}"));
                }
                if let Some(text) = &model.matrix {
                    ui.label(format!("󰘨 {text}"));
                }
            });
        });
}

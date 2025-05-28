//! Each individual's profile card

use crate::app::APP_URL;
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
            .shrink_to_fit();
        ui.add(image);
        ui.separator();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label(&model.name_primary);
                    ui.label(extract_string(model.name_supplementary.as_ref()));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(format!("{}届", model.class_of));
                });
            });
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label(extract_string(model.bio.as_ref()));
                });
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
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    if let Some(text) = &model.wechat {
                        ui.label(format!("󰘑 {text}"));
                    }
                    if let Some(text) = &model.qq {
                        ui.label(format!("󰘅 {text}"));
                    }
                });
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    if let Some(text) = &model.email {
                        ui.label(format!("󰇮 {text}"));
                    }
                    if let Some(text) = &model.matrix {
                        ui.label(format!("󰘨 {text}"));
                    }
                });
            });
        });
}

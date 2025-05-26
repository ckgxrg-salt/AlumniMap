//! Each individual's profile card

use crate::app::APP_URL;
use entity::profile;

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
                    ui.label(model.name_primary.clone());
                    ui.label(model.name_supplementary.clone().unwrap_or_default());
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(format!("{}届", model.class_of));
                });
            });
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.label(model.bio.clone().unwrap_or_default());
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(model.major.clone().unwrap_or_default());
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
                    if let Some(text) = model.wechat.clone() {
                        ui.label(format!("󰘑 {text}"));
                    }
                    if let Some(text) = model.qq.clone() {
                        ui.label(format!("󰘅 {text}"));
                    }
                });
                ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                    if let Some(text) = model.email.clone() {
                        ui.label(format!("󰇮 {text}"));
                    }
                    if let Some(text) = model.matrix.clone() {
                        ui.label(format!("󰘨 {text}"));
                    }
                });
            });
        });
}

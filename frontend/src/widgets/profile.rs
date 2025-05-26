//! Each individual's profile card

use entity::profile;

/// A list contains many profile cards
pub struct ProfileCard {
    inner: profile::Model,
    current_url: String,
}

/// Data manipulation
impl ProfileCard {
    /// Convert from raw model
    pub fn convert(value: profile::Model, current_url: String) -> Self {
        Self {
            inner: value,
            current_url,
        }
    }
}

/// Graphics
impl ProfileCard {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let image = egui::Image::new(format!(
                "{}static/avatars/{}",
                self.current_url, self.inner.avatar
            ))
            .shrink_to_fit();
            ui.add(image);
            ui.separator();
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                        ui.label(self.inner.name_primary.clone());
                        ui.label(self.inner.name_supplementary.clone().unwrap_or_default());
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        ui.label(format!("{}届", self.inner.class_of));
                    });
                });
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                        ui.label(self.inner.bio.clone().unwrap_or_default());
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                        ui.label(self.inner.major.clone().unwrap_or_default());
                    });
                });
                ui.separator();
                self.render_contacts(ui);
            });
        });
        ui.separator();
    }

    /// Draws the collapsible contacts section
    fn render_contacts(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("󰛋".to_string())
            .id_salt(self.inner.id)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        if let Some(text) = self.inner.wechat.clone() {
                            ui.label(format!("󰘑 {text}"));
                        }
                        if let Some(text) = self.inner.qq.clone() {
                            ui.label(format!("󰘅 {text}"));
                        }
                    });
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        if let Some(text) = self.inner.email.clone() {
                            ui.label(format!("󰇮 {text}"));
                        }
                        if let Some(text) = self.inner.matrix.clone() {
                            ui.label(format!("󰘨 {text}"));
                        }
                    });
                });
            });
    }
}

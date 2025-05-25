//! Each individual's profile card

use std::sync::{Arc, Mutex};

use entity::profile;

/// A list contains many profile cards
pub struct ProfileCard {
    inner: profile::Model,
    university_name: String,
    fetch_state: Arc<Mutex<State>>,
}
/// The state of fetching data from the backend
enum State {
    Init,
    Loading,
    Fetched(ehttp::Result<ehttp::Response>),
    Done,
}

impl From<profile::Model> for ProfileCard {
    fn from(value: profile::Model) -> Self {
        Self {
            inner: value,
            university_name: "Loading...".to_string(),
            fetch_state: Arc::new(Mutex::new(State::Init)),
        }
    }
}

/// Data manipulation
impl ProfileCard {
    /// Fetches data from the database
    fn fetch_data(&mut self, ctx: &egui::Context) {
        let should_fetch = {
            let fetch_state: &State = &self.fetch_state.lock().unwrap();
            matches!(fetch_state, State::Init)
        };
        if should_fetch {
            let temp_state = self.fetch_state.clone();
            *temp_state.lock().unwrap() = State::Loading;
            let req = ehttp::Request::get(format!(
                "http://127.0.0.1:8080/api/universities/{}",
                self.inner.university_id
            ));
            ehttp::fetch(req, move |response| {
                *temp_state.lock().unwrap() = State::Fetched(response);
            });
        }
        let response = {
            let fetch_state: &State = &self.fetch_state.lock().unwrap();
            if let State::Fetched(response) = fetch_state {
                Some(response.clone())
            } else {
                None
            }
        };
        if let Some(res) = response {
            if let Ok(val) = res {
                let str = val.text();
                if let Some(text) = str {
                    self.university_name = text.to_string();
                } else {
                    self.university_name = "Unknown University".to_string();
                }
                ctx.request_repaint();
                *self.fetch_state.lock().unwrap() = State::Done;
            } else {
                // Continue to try
                *self.fetch_state.lock().unwrap() = State::Init;
            }
        }
    }
}

/// Graphics
impl ProfileCard {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.fetch_data(ui.ctx());
        ui.horizontal(|ui| {
            let image = egui::Image::new(format!(
                "http://127.0.0.1:8080/static/avatars/{}",
                self.inner.avatar
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
                //ui.label(self.university_name.clone());
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

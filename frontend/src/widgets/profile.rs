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
            ui.image(format!(
                "http://127.0.0.1:8080/static/avatars/{}",
                self.inner.avatar
            ));
            ui.separator();
            ui.vertical(|ui| {
                ui.label(format!(
                    "{} {}          {}届",
                    self.inner.name_primary.clone(),
                    self.inner.name_supplementary.clone().unwrap_or_default(),
                    self.inner.class_of
                ));
                //ui.label(self.university_name.clone());
                ui.label(self.inner.major.clone().unwrap_or_default());
                ui.separator();
                ui.label(self.inner.bio.clone().unwrap_or_default());
            });
        });
    }
}

//! Pops up a list when any point is clicked

use egui::Pos2;
use std::sync::{Arc, Mutex};

use entity::profile;

/// A list contains many profile cards
pub struct List {
    profiles: Vec<profile::Model>,
    pub title: String,
    pub uni_id: i32,
    starting_pos: Pos2,
    fetch_state: Arc<Mutex<State>>,
}
/// The state of fetching data from the backend
enum State {
    Init,
    Loading,
    Fetched(ehttp::Result<ehttp::Response>),
    Done,
}

/// Data manipulation
impl List {
    /// Creates a new list
    pub fn new(title: String, uni_id: i32, starting_pos: Pos2) -> Self {
        Self {
            profiles: Vec::new(),
            title,
            uni_id,
            starting_pos,
            fetch_state: Arc::new(Mutex::new(State::Init)),
        }
    }

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
                "http://127.0.0.1:8080/api/profiles/{}",
                self.uni_id
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
                let str: String = val.json().unwrap_or_default();
                if let Ok(parsed) = serde_json::from_str::<Vec<profile::Model>>(&str) {
                    self.profiles.extend(parsed);
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
impl List {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ctx: &egui::Context, should_display: &mut bool) {
        self.fetch_data(ctx);

        let window = egui::Window::new(self.title.clone())
            .default_pos(self.starting_pos)
            .collapsible(false)
            .open(should_display);
        window.show(ctx, |ui| {
            for each in &self.profiles {
                ui.horizontal(|ui| {
                    ui.label("Pic Here");
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label(each.name_primary.clone());
                        ui.label(each.name_supplementary.clone().unwrap_or_default());
                        ui.separator();
                        ui.label(each.bio.clone().unwrap_or_default());
                    });
                });
            }
        });
    }
}

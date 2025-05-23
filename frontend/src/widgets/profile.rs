//! Pops up a list when any point is clicked

use egui::Pos2;
use std::sync::{Arc, Mutex};

use entity::profile;

/// Refer to the backend code for this struct
struct Profile {
    name_primary: String,
    name_supplementary: Option<String>,
    class_of: i32,
    avatar: String,
    bio: Option<String>,
    major: Option<String>,
    qq: Option<String>,
    wechat: Option<String>,
    email: Option<String>,
    matrix: Option<String>,
}

impl From<profile::Model> for Profile {
    fn from(value: profile::Model) -> Self {
        Self {
            name_primary: value.name_primary,
            name_supplementary: value.name_supplementary,
            class_of: value.class_of,
            avatar: value.avatar,
            bio: value.bio,
            major: value.major,
            qq: value.qq,
            wechat: value.wechat,
            email: value.email,
            matrix: value.matrix,
        }
    }
}

/// A list contains many profile cards
pub struct List {
    profiles: Vec<Profile>,
    title: String,
    uni_id: i32,
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
    fn fetch_data(&mut self, ui: &mut egui::Ui) {
        let should_fetch = {
            let fetch_state: &State = &self.fetch_state.lock().unwrap();
            matches!(fetch_state, State::Init)
        };
        if should_fetch {
            let temp_state = self.fetch_state.clone();
            *temp_state.lock().unwrap() = State::Loading;
            let req = ehttp::Request::get(format!(
                "http://127.0.0.1:8080/api/profiles/{0}",
                self.uni_id
            ));
            ehttp::fetch(req, move |response| {
                *temp_state.lock().unwrap() = State::Fetched(response);
            });
        }
        let should_loading = {
            let fetch_state: &State = &self.fetch_state.lock().unwrap();
            matches!(fetch_state, State::Loading)
        };
        if should_loading {
            ui.label("Loading...");
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
                    let points = parsed.into_iter().map(Profile::from);
                    self.profiles.extend(points);
                }
                ui.ctx().request_repaint();
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
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.fetch_data(ui);

        let mut should_display = true;
        let window = egui::Window::new(self.title.clone())
            .default_pos(self.starting_pos)
            .collapsible(false)
            .open(&mut should_display);
        window.show(ui.ctx(), |ui| {
            for each in &self.profiles {
                each.render(ui);
            }
        });
    }
}

impl Profile {
    /// Draw a single profile card
    fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Pic Here");
            ui.separator();
            ui.vertical(|ui| {
                ui.label(self.name_primary.clone());
                ui.label(self.name_supplementary.clone().unwrap_or_default());
                ui.separator();
                ui.label(self.bio.clone().unwrap_or_default());
            });
        });
    }
}

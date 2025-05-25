//! Pops up a list when any point is clicked

use egui::Pos2;
use std::sync::{Arc, Mutex};

use crate::widgets::profile::ProfileCard;
use entity::profile;

/// A list contains many profile cards
pub struct List {
    profiles: Vec<ProfileCard>,
    pub title: String,
    pub uni_id: i32,
    starting_pos: Pos2,
    current_url: String,
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
    pub fn new(title: String, uni_id: i32, starting_pos: Pos2, current_url: String) -> Self {
        Self {
            profiles: Vec::new(),
            title,
            uni_id,
            starting_pos,
            current_url,
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
            let req =
                ehttp::Request::get(format!("{}api/profiles/{}", self.current_url, self.uni_id));
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
                    let profiles = parsed.into_iter().map(|orig: profile::Model| {
                        ProfileCard::convert(orig, self.current_url.clone())
                    });
                    self.profiles.extend(profiles);
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
            .current_pos(self.starting_pos)
            .collapsible(false)
            .open(should_display);
        window.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for each in &mut self.profiles {
                    each.render(ui);
                }
            });
        });
    }
}

//! Pops up a list when any point is clicked

use egui::Pos2;

use crate::app::APP_URL;
use crate::fetcher::FetchedData;
use crate::widgets::card;
use entity::profile;

/// A list contains many profile cards
pub struct List {
    profiles: FetchedData<Vec<profile::Model>>,
    pub title: String,
    pub uni_id: i32,
    starting_pos: Pos2,
}

/// Data manipulation
impl List {
    /// Creates a new list
    pub fn new(title: String, uni_id: i32, starting_pos: Pos2) -> Self {
        let profiles = FetchedData::new(format!("{}api/profiles/{uni_id}", *APP_URL), |response| {
            let str: String = response.json().unwrap_or_default();
            serde_json::from_str::<Vec<profile::Model>>(&str).ok()
        });
        Self {
            profiles,
            title,
            uni_id,
            starting_pos,
        }
    }
}

/// Graphics
impl List {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ctx: &egui::Context, should_display: &mut bool) {
        self.profiles.poll(ctx);

        let window = egui::Window::new(self.title.clone())
            .current_pos(self.starting_pos)
            .collapsible(false)
            .open(should_display);
        window.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(data) = &self.profiles.data {
                    for each in data {
                        card::render(each, ui);
                    }
                }
            });
        });
    }
}

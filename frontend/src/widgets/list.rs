//! Pops up a list when any point is clicked

use egui::Pos2;

use crate::fetcher::FetchedData;
use crate::widgets::profile::ProfileCard;
use entity::profile;

/// A list contains many profile cards
pub struct List {
    profiles: FetchedData<Vec<profile::Model>>,
    pub title: String,
    pub uni_id: i32,
    starting_pos: Pos2,
    current_url: String,
}

/// Data manipulation
impl List {
    /// Creates a new list
    pub fn new(title: String, uni_id: i32, starting_pos: Pos2, current_url: String) -> Self {
        let profiles = FetchedData::new(
            format!("{current_url}api/universities{uni_id}"),
            |response| {
                let str: String = response.json().unwrap_or_default();
                if let Ok(parsed) = serde_json::from_str::<Vec<profile::Model>>(&str) {
                    parsed
                        .into_iter()
                        .map(|orig| ProfileCard::convert(orig, current_url));
                    return Some(parsed);
                }
                None
            },
        );
        Self {
            profiles,
            title,
            uni_id,
            starting_pos,
            current_url,
        }
    }
}

/// Graphics
impl List {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ctx: &egui::Context, should_display: &mut bool) {
        self.profiles.poll();

        let window = egui::Window::new(self.title.clone())
            .current_pos(self.starting_pos)
            .collapsible(false)
            .open(should_display);
        window.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(data) = &mut self.profiles.data {
                    for each in data {
                        each.render(ui);
                    }
                }
            });
        });
    }
}

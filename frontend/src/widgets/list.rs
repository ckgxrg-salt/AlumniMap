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
}
/// Keep track of the [`List`]'s state
pub struct ListState {
    pub open: bool,
    initial_pos: Option<Pos2>,
    pub inner: List,
}

/// Data manipulation
impl ListState {
    /// Creates a new list
    pub fn new(title: String, uni_id: i32, initial_pos: Pos2) -> Self {
        let profiles = FetchedData::new(format!("{}api/profiles/{uni_id}", *APP_URL), |response| {
            let str: String = response.json().unwrap_or_default();
            serde_json::from_str::<Vec<profile::Model>>(&str).ok()
        });
        let list = List {
            profiles,
            title,
            uni_id,
        };
        Self {
            open: true,
            initial_pos: Some(initial_pos),
            inner: list,
        }
    }
}

/// Graphics
impl ListState {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ctx: &egui::Context) {
        self.inner.profiles.poll(ctx);

        let mut window = egui::Window::new(&self.inner.title)
            .collapsible(true)
            .open(&mut self.open);

        // Move to new position when reopened
        if let Some(pos) = self.initial_pos.take() {
            window = window.current_pos(pos);
        }

        window.show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(data) = &self.inner.profiles.data {
                    for each in data {
                        card::render(each, ui);
                    }
                }
            });
        });
    }
}

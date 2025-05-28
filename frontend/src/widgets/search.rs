use crate::app::APP_URL;
use crate::fetcher::FetchedData;
use crate::widgets::map::WorldMap;
use egui::FontId;
use entity::university;

pub struct Search {
    fetcher: Option<FetchedData<Vec<university::Model>>>,
    text: String,
}

/// Data manipulation
impl Search {
    pub fn new() -> Self {
        Self {
            fetcher: None,
            text: String::new(),
        }
    }
}

/// Graphics
impl Search {
    pub fn render(&mut self, ui: &mut egui::Ui, world_map: &mut WorldMap) {
        ui.horizontal(|ui| {
            let input_response = ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .hint_text("Search...")
                    .font(FontId::proportional(24.0)),
            );
            let button_response = ui.button(" ");
            if input_response.lost_focus() && (ui.input(|i| i.key_pressed(egui::Key::Enter)))
                || button_response.clicked()
            {
                self.fetcher = Some(FetchedData::new(
                    format!("{}api/search/universities/{}", *APP_URL, self.text),
                    |response| {
                        let str: String = response.json().unwrap_or_default();
                        serde_json::from_str::<Vec<university::Model>>(&str).ok()
                    },
                ));
                self.text = String::new();
            }

            if let Some(fetching) = &mut self.fetcher {
                fetching.poll(ui.ctx());
                if let Some(data) = &mut fetching.data {
                    world_map.highlights.append(data);
                    self.fetcher = None;
                }
            }
        });
    }
}

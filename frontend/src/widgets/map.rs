//! Draw points and lines on a world map

use egui::{Color32, Pos2, Rect, Vec2};
use std::sync::{Arc, Mutex};

use crate::widgets::list::List;
use entity::university;

/// The world map on the main interface
pub struct WorldMap {
    /// All dest [`Point`]s will draw a line from the base [`Point`]
    base: university::Model,
    dests: Vec<university::Model>,
    internal_area: Rect,
    image_url: String,
    fetch_state: Arc<Mutex<State>>,

    /// All currently visible [`List`]s
    popups: Vec<(List, bool)>,
}
/// The state of fetching data from the backend
enum State {
    Init,
    Loading,
    Fetched(ehttp::Result<ehttp::Response>),
    Done,
}

/// Data manipulation
impl WorldMap {
    /// Creates a new world map
    pub fn new(image_url: String, base: university::Model) -> Self {
        Self {
            base,
            dests: Vec::new(),
            internal_area: Rect::ZERO,
            image_url,
            fetch_state: Arc::new(Mutex::new(State::Init)),
            popups: Vec::new(),
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
            let req = ehttp::Request::get("http://127.0.0.1:8080/api/universities");
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
                if let Ok(parsed) = serde_json::from_str::<Vec<university::Model>>(&str) {
                    self.dests.extend(parsed);
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

/// Render related
impl WorldMap {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.fetch_data(ui);

        // Map itself
        let mut real_internal_area = self.internal_area;
        let scene = egui::Scene::new().zoom_range(0.1..=10.0);
        scene.show(ui, &mut real_internal_area, |ui| {
            let image = egui::Image::new("http://127.0.0.1:8080/static/world.svg")
                .sense(egui::Sense::CLICK | egui::Sense::HOVER)
                .fit_to_original_size(1.0);
            let image_res = ui.add(image);
            let area = image_res.rect;
            self.draw_base_and_lines(ui, area);
            self.draw_points(ui, area);
            if let Some(click_pos) = image_res.interact_pointer_pos() {
                self.check_click(ui, click_pos, area);
            }
            if let Some(hover_pos) = image_res.hover_pos() {
                self.check_hover(ui, hover_pos, area);
            }
        });
        self.internal_area = real_internal_area;

        // Popups
        let mut closing = Vec::new();
        for (index, each) in self.popups.iter_mut().enumerate() {
            let (list, should_display) = each;
            list.render(ui.ctx(), should_display);
            if !*should_display {
                closing.push(index);
            }
        }
        for each in closing {
            self.popups.remove(each);
        }
    }

    /// Draws the base point and all lines from the base to the dests
    fn draw_base_and_lines(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        let base_pos = to_ui_coords(
            to_norm_coords(self.base.longitude, self.base.latitude),
            area,
        );
        for each in &self.dests {
            let dest_pos = to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
            painter.line_segment(
                [base_pos, dest_pos],
                egui::Stroke::new(1.0, Color32::from_hex(&each.colour).unwrap_or_default()),
            );
        }
        painter.circle(
            base_pos,
            7.0,
            Color32::from_hex(&self.base.colour).unwrap_or_default(),
            egui::Stroke::new(
                1.0,
                Color32::from_hex(&self.base.colour).unwrap_or_default(),
            ),
        );
    }

    /// Recursively draws all destination points
    fn draw_points(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        for each in &self.dests {
            let draw_pos = to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
            painter.circle(
                draw_pos,
                5.0 / area.height() * self.internal_area.height(),
                Color32::from_hex(&each.colour).unwrap_or_default(),
                egui::Stroke::new(1.0, Color32::from_hex(&each.colour).unwrap_or_default()),
            );
        }
    }

    /// Handles the logic when a destination point is clicked
    fn check_click(&mut self, ui: &egui::Ui, click_pos: Pos2, area: Rect) {
        for each in &self.dests {
            let norm_coord = Pos2::new(
                (click_pos.x - area.left()) / area.width(),
                (click_pos.y - area.top()) / area.height(),
            );
            let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
            if distance < 5.0 / area.height() / area.height() * self.internal_area.height()
                && !self.popups.iter().any(|list| list.0.uni_id == each.id)
            {
                let starting_pos = ui
                    .input(|input| input.pointer.interact_pos())
                    .unwrap_or_default();
                let popup = List::new(each.title.clone(), each.id, starting_pos);
                self.popups.push((popup, true));
            }
        }
    }

    /// Handles the logic when the cursor hovers over a destination point
    fn check_hover(&self, ui: &egui::Ui, hover_pos: Pos2, area: Rect) {
        for each in &self.dests {
            let norm_coord = Pos2::new(
                (hover_pos.x - area.left()) / area.width(),
                (hover_pos.y - area.top()) / area.height(),
            );
            let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
            if distance < 5.0 / area.height() / area.height() * self.internal_area.height() {
                egui::show_tooltip_at_pointer(
                    ui.ctx(),
                    ui.layer_id(),
                    egui::Id::new("dest_points_tooltip"),
                    |ui| {
                        ui.label(each.title.clone());
                    },
                );
            }
        }
    }
}

/// Converts longitude and latitude to normalised coordinates
fn to_norm_coords(longitude: f32, latitude: f32) -> Pos2 {
    Pos2::new(longitude / 360.0 + 0.5, 0.5 - latitude / 180.0)
}

/// Converts normalised coordinates to ui coordinates
fn to_ui_coords(norm: Pos2, area: Rect) -> Pos2 {
    Pos2::new(
        norm.x * area.width() + area.left(),
        norm.y * area.height() + area.top(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_conversion() {
        assert_eq!(to_norm_coords(180.0, 90.0), Pos2::new(1.0, 0.0));
        assert_eq!(to_norm_coords(90.0, 45.0), Pos2::new(0.75, 0.25));
        assert_eq!(to_norm_coords(0.0, 0.0), Pos2::new(0.5, 0.5));
        assert_eq!(to_norm_coords(-180.0, -90.0), Pos2::new(0.0, 1.0));
        assert_eq!(to_norm_coords(-90.0, -45.0), Pos2::new(0.25, 0.75));
    }
}

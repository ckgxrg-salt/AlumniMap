//! Draw points and lines on a world map

use egui::{Color32, Pos2, Rect};
use std::sync::{Arc, Mutex};

use entity::university;

/// A points on the map that represents a destination
/// This is simpler than [`entity::university::Model`]
pub struct Point {
    /// Positive value for eastern longitude, negative for western
    longitude: f32,
    /// Positive value for northern latitude, negative for southern
    latitude: f32,
    name: String,
    colour: Color32,
}

impl Point {
    /// Creates a new destination point
    pub fn new(longitude: f32, latitude: f32, name: String, colour: Color32) -> Self {
        Self {
            longitude,
            latitude,
            name,
            colour,
        }
    }
}

impl From<university::Model> for Point {
    fn from(value: university::Model) -> Self {
        Self {
            name: value.title,
            colour: Color32::from_hex(&value.colour).unwrap_or(Color32::DARK_RED),
            longitude: value.longitude,
            latitude: value.latitude,
        }
    }
}

/// The world map on the main interface
pub struct WorldMap {
    // All dest [Point]s will draw a line from the base [Point]
    base: Point,
    dests: Vec<Point>,
    internal_area: Rect,
    image_url: String,
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
impl WorldMap {
    /// Creates a new world map
    pub fn new(image_url: String, base: Point) -> Self {
        Self {
            base,
            dests: Vec::new(),
            internal_area: Rect::ZERO,
            image_url,
            fetch_state: Arc::new(Mutex::new(State::Init)),
        }
    }

    /// Adds a destination point to the map
    pub fn add_destination(mut self, point: Point) -> Self {
        self.dests.push(point);
        self
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
        if let Some(Ok(res)) = response {
            let str: String = res.json().unwrap_or_default();
            if let Ok(parsed) = serde_json::from_str::<Vec<university::Model>>(&str) {
                let points = parsed.into_iter().map(Point::from);
                self.dests.extend(points);
            }
            ui.ctx().request_repaint();
            *self.fetch_state.lock().unwrap() = State::Done;
        }
    }
}

/// Render related
impl WorldMap {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.fetch_data(ui);

        let mut real_internal_area = self.internal_area;
        let scene = egui::Scene::new().zoom_range(1.0..=10.0);
        scene.show(ui, &mut real_internal_area, |ui| {
            let image = egui::Image::new("http://127.0.0.1:8080/static/world.svg")
                .sense(egui::Sense::CLICK | egui::Sense::HOVER);
            let image_res = ui.add(image);
            let area = image_res.rect;
            self.draw_base_and_lines(ui, area);
            self.draw_points(ui, area);
            if let Some(click_pos) = image_res.interact_pointer_pos() {
                self.check_click(click_pos, area);
            }
            if let Some(hover_pos) = image_res.hover_pos() {
                self.check_hover(ui, hover_pos, area);
            }
        });
        self.internal_area = real_internal_area;
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
            painter.line_segment([base_pos, dest_pos], egui::Stroke::new(1.0, each.colour));
        }
        painter.circle(
            base_pos,
            7.0,
            self.base.colour,
            egui::Stroke::new(1.0, self.base.colour),
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
                each.colour,
                egui::Stroke::new(1.0, each.colour),
            );
        }
    }

    /// Handles the logic when a destination point is clicked
    fn check_click(&self, click_pos: Pos2, area: Rect) {
        for each in &self.dests {
            let norm_coord = Pos2::new(
                (click_pos.x - area.left()) / area.width(),
                (click_pos.y - area.top()) / area.height(),
            );
            let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
            if distance < 5.0 / area.height() / area.height() * self.internal_area.height() {
                todo!("Click on {}", each.name);
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
                        ui.label(each.name.clone());
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

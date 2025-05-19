//! Describes how to draw points and lines on a map

use egui::{Color32, Pos2, Rect};

/// A points on the map that represents a destination
pub struct DestnationPoint {
    /// Positive value for eastern longitude, negative for western
    longitude: f32,
    /// Positive value for northern latitude, negative for southern
    latitude: f32,
    name: String,
}

/// The world map on the main interface
pub struct WorldMap {
    dests: Vec<DestnationPoint>,
    image_url: String,
}

impl WorldMap {
    /// Calls egui to draw everything to the screen
    pub fn render(&self, ui: &mut egui::Ui) {
        let image_res = ui.image(self.image_url.clone());
        let area = image_res.rect;
        self.draw_points(ui, area);
        if let Some(click_pos) = image_res.interact_pointer_pos() {
            self.check_click(click_pos, area);
        }
        if let Some(hover_pos) = image_res.hover_pos() {
            self.check_hover(hover_pos, area);
        }
    }

    /// Recursively draws all destination points
    fn draw_points(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        for each in &self.dests {
            let centre = to_norm_coords(each.longitude, each.latitude);
            let draw_pos = to_ui_coords(centre, area);
            painter.circle(
                draw_pos,
                15.0,
                Color32::RED, //TODO: Find a way to load colour
                egui::Stroke::new(1.0, Color32::RED),
            );
        }
    }

    /// Handles the logic when a destination point is clicked
    fn check_click(&self, click_pos: Pos2, area: Rect) {
        for each in &self.dests {
            let norm_coord = Pos2::new(
                (click_pos.x - area.left()) / area.width(),
                (click_pos.y - area.right()) / area.height(),
            );
            let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
            if distance < 15.0 / area.height() {
                log::info!("Click on {}", each.name);
            }
        }
    }

    /// Handles the logic when the cursor hovers over a destination point
    fn check_hover(&self, hover_pos: Pos2, area: Rect) {
        for each in &self.dests {
            let norm_coord = Pos2::new(
                (hover_pos.x - area.left()) / area.width(),
                (hover_pos.y - area.right()) / area.height(),
            );
            let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
            if distance < 15.0 / area.height() {
                log::info!("Hover on {}", each.name);
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

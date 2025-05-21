//! Draw points and lines on a world map

use egui::{Color32, Pos2, Rect};

/// A points on the map that represents a destination
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

/// The world map on the main interface
pub struct WorldMap {
    // All dest [Point]s will draw a line from the base [Point]
    base: Point,
    dests: Vec<Point>,
    image_url: String,
}

/// Data manipulation
impl WorldMap {
    /// Creates a new world map
    pub fn new(image_url: String, base: Point) -> Self {
        Self {
            base,
            dests: Vec::new(),
            image_url,
        }
    }

    /// Adds a destination point to the map
    pub fn add_destination(mut self, point: Point) -> Self {
        self.dests.push(point);
        self
    }
}

/// Render relates
impl WorldMap {
    /// Calls egui to draw everything to the screen
    pub fn render(&self, ui: &mut egui::Ui) {
        // TODO: Images must be served via static assets from the backend
        let image = egui::Image::new(egui::include_image!("../../assets/world.svg"))
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
    }

    /// Draws the base point and all lines from the base to the dests
    fn draw_base_and_lines(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        let base_pos = to_ui_coords(
            to_norm_coords(self.base.longitude, self.base.latitude),
            area,
        );
        painter.circle(
            base_pos,
            15.0,
            self.base.colour,
            egui::Stroke::new(1.0, self.base.colour),
        );
        for each in &self.dests {
            let dest_pos = to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
            painter.line_segment([base_pos, dest_pos], egui::Stroke::new(1.0, each.colour));
        }
    }

    /// Recursively draws all destination points
    fn draw_points(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        for each in &self.dests {
            let draw_pos = to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
            painter.circle(
                draw_pos,
                15.0,
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
            if distance < 15.0 / area.height() {
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
            if distance < 15.0 / area.height() {
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

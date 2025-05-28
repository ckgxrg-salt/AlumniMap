//! Draw points and lines on a world map

use egui::{Color32, Pos2, Rect};

use crate::app::APP_URL;
use crate::fetcher::FetchedData;
use crate::widgets::list::ListState;
use entity::university;

/// Defines the world map image to use
/// The offset value will be added to the provided value in order to translate the map if it's not
/// centered at the 0 degree longitude.
const LONGITUDE_OFFSET: f32 = -170.0;
const LATITUDE_OFFSET: f32 = -13.0;
const IMAGE: egui::ImageSource<'_> = egui::include_image!("../../assets/world.png");

/// The world map on the main interface
pub struct WorldMap {
    /// All dests will draw a line from the base point
    base: FetchedData<university::Model>,
    dests: FetchedData<Vec<university::Model>>,
    internal_area: Rect,

    /// All currently visible [`List`]s
    popups: Vec<ListState>,
}

/// Data manipulation
impl WorldMap {
    /// Creates a new world map
    pub fn new() -> Self {
        let base = FetchedData::new(format!("{}api/base", *APP_URL), |response| {
            let str: String = response.json().unwrap_or_default();
            serde_json::from_str::<university::Model>(&str).ok()
        });
        let dests = FetchedData::new(format!("{}api/universities", *APP_URL), |response| {
            let str: String = response.json().unwrap_or_default();
            serde_json::from_str::<Vec<university::Model>>(&str).ok()
        });
        Self {
            base,
            dests,
            internal_area: Rect::ZERO,
            popups: Vec::new(),
        }
    }
}

/// Render related
impl WorldMap {
    /// Calls egui to draw everything to the screen
    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.base.poll(ui.ctx());
        self.dests.poll(ui.ctx());

        // Map itself
        let mut real_internal_area = self.internal_area;
        let scene = egui::Scene::new().zoom_range(0.1..=10.0);
        scene.show(ui, &mut real_internal_area, |ui| {
            let image = egui::Image::new(IMAGE)
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
            each.render(ui.ctx());
            if !each.open {
                closing.push(index);
            }
        }
        for each in closing {
            self.popups.remove(each);
        }
    }

    /// Draws the base point and all lines from the base to the dests
    fn draw_base_and_lines(&self, ui: &egui::Ui, area: Rect) {
        if let Some(data) = &self.base.data {
            let painter = ui.painter();
            let base_pos = to_ui_coords(to_norm_coords(data.longitude, data.latitude), area);
            if let Some(data) = &self.dests.data {
                for each in data {
                    let dest_pos =
                        to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
                    painter.line_segment(
                        [base_pos, dest_pos],
                        egui::Stroke::new(1.0, Color32::from_hex(&each.colour).unwrap_or_default()),
                    );
                }
            }
            painter.circle(
                base_pos,
                7.0,
                Color32::from_hex(&data.colour).unwrap_or_default(),
                egui::Stroke::new(1.0, Color32::from_hex(&data.colour).unwrap_or_default()),
            );
        }
    }

    /// Recursively draws all destination points
    fn draw_points(&self, ui: &egui::Ui, area: Rect) {
        let painter = ui.painter();
        if let Some(data) = &self.dests.data {
            for each in data {
                let draw_pos = to_ui_coords(to_norm_coords(each.longitude, each.latitude), area);
                painter.circle(
                    draw_pos,
                    5.0 / area.height() * self.internal_area.height(),
                    Color32::from_hex(&each.colour).unwrap_or_default(),
                    egui::Stroke::new(1.0, Color32::from_hex(&each.colour).unwrap_or_default()),
                );
            }
        }
    }

    /// Handles the logic when a destination point is clicked
    fn check_click(&mut self, ui: &egui::Ui, click_pos: Pos2, area: Rect) {
        if let Some(data) = &self.dests.data {
            for each in data {
                let norm_coord = Pos2::new(
                    (click_pos.x - area.left()) / area.width(),
                    (click_pos.y - area.top()) / area.height(),
                );
                let distance = norm_coord.distance(to_norm_coords(each.longitude, each.latitude));
                if distance < 5.0 / area.height() / area.height() * self.internal_area.height()
                    && !self.popups.iter().any(|list| list.inner.uni_id == each.id)
                {
                    let initial_pos = ui
                        .input(|input| input.pointer.interact_pos())
                        .unwrap_or_default();
                    let popup = ListState::new(each.title.clone(), each.id, initial_pos);
                    self.popups.push(popup);
                }
            }
        }
    }

    /// Handles the logic when the cursor hovers over a destination point
    fn check_hover(&self, ui: &egui::Ui, hover_pos: Pos2, area: Rect) {
        if let Some(data) = &self.dests.data {
            for each in data {
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
                            ui.label(&each.title);
                        },
                    );
                }
            }
        }
    }
}

/// Translates longitude by the offset value
/// Will wrap from 180 to -180 if exceeds the boundary
fn offset_longitude(longitude: f32) -> f32 {
    let result = longitude + LONGITUDE_OFFSET;
    if result > 180.0 {
        result - 180.0
    } else if result < -180.0 {
        result + 180.0
    } else {
        result
    }
}
/// Translates latitude by the offset value
/// Will wrap from 90 to -90 if exceeds the boundary
fn offset_latitude(latitude: f32) -> f32 {
    let result = latitude + LATITUDE_OFFSET;
    if result > 90.0 {
        result - 90.0
    } else if result < -90.0 {
        result + 90.0
    } else {
        result
    }
}

/// Converts longitude and latitude to normalised coordinates
///
/// The offset value will be added to the provided value in order to translate the map if it's not
/// centered at the 0 degree longitude.
fn to_norm_coords(longitude: f32, latitude: f32) -> Pos2 {
    Pos2::new(
        offset_longitude(longitude) / 360.0 + 0.5,
        0.5 - offset_latitude(latitude) / 180.0,
    )
}

/// Converts normalised coordinates to ui coordinates
fn to_ui_coords(norm: Pos2, area: Rect) -> Pos2 {
    Pos2::new(
        norm.x * area.width() + area.left(),
        norm.y * area.height() + area.top(),
    )
}

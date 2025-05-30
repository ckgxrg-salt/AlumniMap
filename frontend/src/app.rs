use std::sync::LazyLock;

use crate::init;
use crate::widgets::map::WorldMap;
use crate::widgets::search::Search;

pub static APP_URL: LazyLock<String> = LazyLock::new(get_app_url);
fn get_app_url() -> String {
    web_sys::window()
        .expect("Cannot get current window object, do your browser support it?")
        .location()
        .href()
        .expect("Cannot get current href, do your browser support it?")
}

pub struct AlumniMapApp {
    world_map: WorldMap,
    search: Search,
}

impl AlumniMapApp {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        init::init_font(&cc.egui_ctx);
        init::set_theme(&cc.egui_ctx);
        Self {
            world_map: WorldMap::new(),
            search: Search::new(),
        }
    }
}

impl eframe::App for AlumniMapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.search.render(ui, &mut self.world_map);
            ui.separator();
            self.world_map.render(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                credits(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.hyperlink_to(
            "GPLv3 License.",
            "https://www.gnu.org/licenses/gpl-3.0.en.html",
        );
        ui.label("Free software under");
        ui.hyperlink_to("ckgxrg.", "https://ckgxrg.io");
        ui.label("by");
        ui.hyperlink_to("AlumniMap", "https://github.com/ckgxrg-salt/Alumnimap");
        ui.spacing_mut().item_spacing.x = 0.0;
    });
}

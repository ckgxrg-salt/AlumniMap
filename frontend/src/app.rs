use crate::init;
use crate::widgets::map::WorldMap;

pub struct AlumniMapApp {
    world_map: WorldMap,
}

impl AlumniMapApp {
    #[cfg(not(target_arch = "wasm32"))]
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        init::init_font(&cc.egui_ctx);
        Self {
            world_map: init::create_world_map(String::new()),
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        init::init_font(&cc.egui_ctx);
        let url = web_sys::window().unwrap().location().href().unwrap();
        Self {
            world_map: init::create_world_map(url),
        }
    }
}

impl eframe::App for AlumniMapApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.separator();

            self.world_map.render(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                credits(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn credits(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.hyperlink_to("AlumniMap", "https://github.com/ckgxrg-salt/Alumnimap");
        ui.label(" by ");
        ui.hyperlink_to("ckgxrg", "https://ckgxrg.io");
        ui.label(". Free software under ");
        ui.hyperlink_to(
            "GPLv3 License",
            "https://www.gnu.org/licenses/gpl-3.0.en.html",
        );
        ui.label(".");
    });
}

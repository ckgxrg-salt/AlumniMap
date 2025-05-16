use egui::TextEdit;

#[derive(Default)]
pub struct AlumnimapApp {
    label: String,
}

impl AlumnimapApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Default::default()
    }
}

impl eframe::App for AlumnimapApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

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
            ui.horizontal(|ui| {
                ui.add(TextEdit::singleline(&mut self.label).hint_text("Search..."));
            });

            ui.separator();

            ui.image(egui::include_image!("../assets/map.jpg"));

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
        ui.hyperlink_to("Alumnimap", "https://github.com/ckgxrg-salt/Alumnimap");
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

//! Pops up a list when any point is clicked

use egui::Pos2;

/// Refer to the backend code for this struct
struct Profile {
    name_primary: String,
    name_supplementary: Option<String>,
    class_of: i32,
    avatar: Option<String>,
    profile: Option<String>,
    major: Option<String>,
    contact: Contacts,
}

/// How to contact this individual
struct Contacts {
    qq: Option<String>,
    wechat: Option<String>,
    email: Option<String>,
}

/// A list contains many profile cards
struct List {
    profiles: Vec<Profile>,
    title: String,
}

impl List {
    /// Calls egui to draw everything to the screen
    pub fn render(&self, ctx: &egui::Context, starting_pos: Pos2) {
        let mut should_display = true;
        let window = egui::Window::new(self.title.clone())
            .default_pos(starting_pos)
            .collapsible(false)
            .open(&mut should_display);
        window.show(ctx, |ui| {
            for each in &self.profiles {
                each.render(ui);
            }
        });
    }
}

impl Profile {
    /// Draw a single profile card
    fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Pic Here");
            ui.separator();
            ui.vertical(|ui| {
                ui.label(self.name_primary.clone());
                ui.label(self.name_supplementary.clone().unwrap_or_default());
                ui.separator();
                ui.label(self.profile.clone().unwrap_or_default());
            });
        });
    }
}

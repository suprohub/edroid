use egui::{Response, RichText, Sense, Ui};

use crate::repo::App;

pub fn group_button(ui: &mut Ui, app: &App, image_left: bool) {
    if ui.group(|ui| {
        if image_left {
            ui.horizontal(|ui| {
                if let Some(icon) = &app.icon {
                    log::info!("{icon}");
                    ui.image("https://f-droid.org/repo/icons/".to_string() + icon);
                }
                ui.vertical(|ui| {
                    ui.label(RichText::new(app.name.clone()).strong());
                    if !app.summary.is_empty() {
                        ui.label(&app.summary);
                    }
                });
                ui.allocate_space(egui::Vec2::new(ui.available_width(), 0.0));
            });
        } else {
            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    if let Some(icon) = &app.icon {
                        log::info!("{icon}");
                        ui.image("https://f-droid.org/repo/icons/".to_string() + icon);
                    }
                    ui.label(RichText::new(app.name.clone()).strong());
                });

                if !app.summary.is_empty() {
                    ui.label(&app.summary);
                }
            });
        }     
    })
    .response.interact(Sense::click()).clicked() {
        log::info!("app {} clicked", app.name);
    }
}

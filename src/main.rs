use eframe::NativeOptions;
use log::Level;

pub mod app;
pub mod egui_custom;
pub mod repo;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    eframe::run_native(
        "Edroid",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::Edroid::new(cc)))),
    )
    .unwrap()
}

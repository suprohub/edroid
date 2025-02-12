use eframe::NativeOptions;

pub mod app;
pub mod repo;

fn main() {
    eframe::run_native(
        "Edroid",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::Edroid::new(cc)))),
    )
    .unwrap()
}

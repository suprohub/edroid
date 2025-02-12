use eframe::NativeOptions;

pub mod app;
pub use app::*;

fn main() {
    eframe::run_native(
        "Rust Browser",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BrowserApp::new(cc)))),
    ).unwrap()
}
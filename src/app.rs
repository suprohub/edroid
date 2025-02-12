
use eframe::egui;
use winit::window::Window;

pub struct BrowserApp {

}

impl BrowserApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
        }
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}
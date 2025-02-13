pub mod app;
pub mod egui_custom;
pub mod repo;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use android_logger::{Config, FilterBuilder};
    use log::LevelFilter;

    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Edroid",
        options,
        Box::new(|cc| Ok(Box::new(app::Edroid::new(cc)))),
    )
    .unwrap();
}

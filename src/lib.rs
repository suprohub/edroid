
#[cfg(target_os = "android")]
pub mod app;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use log::LevelFilter;
    use android_logger::{Config, FilterBuilder};

    android_logger::init_once(
        Config::default()
            .with_max_level(LevelFilter::Info)
            .with_filter(FilterBuilder::new().parse("debug,wry=info").build()),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Browser",
        options,
        Box::new(|cc| Ok(Box::new(app::BrowserApp::new(cc)))),
    )
    .unwrap();
}
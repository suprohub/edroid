use std::{
    io::{BufReader, Cursor, Read},
    sync::Arc,
};

use anyhow::Result;
use egui::{Align, Context, Layout, RichText};
use itertools::Itertools;
use jni::objects::{JObject, JString, JValue};
use parking_lot::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use zip::ZipArchive;

use crate::{egui_custom::group_button, repo::Repo};

//const PACKAGE_PATH: &str = "/data/data/me.avidor.edroid/files/";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Edroid {
    #[serde(skip)]
    rt: Runtime,
    #[serde(skip)]
    web_client: Client,
    repos: Arc<Mutex<Vec<Repo>>>,
    layout: LatestAppsLayout,
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub enum LatestAppsLayout {
    #[default]
    Fdroid,
}

impl eframe::App for Edroid {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.heading("Edroid");
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Sync").clicked() {
                        self.sync(ctx);
                    };
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let lock = self.repos.lock();
                let mut apps = lock
                    .iter()
                    .flat_map(|r| {
                        if let Some(apps) = &r.apps {
                            apps.iter().collect::<Vec<_>>()
                        } else {
                            Vec::new()
                        }
                    })
                    .sorted_by_key(|a| date_to_integer(&a.last_updated).map(|i| -i))
                    .take(50);

                match self.layout {
                    LatestAppsLayout::Fdroid => {
                        for row_type in (0..=2u8).cycle() {
                            match row_type {
                                0 => {
                                    if let Some(app) = apps.next() {
                                        group_button(ui, app, true);
                                    } else {
                                        break;
                                    }
                                }
                                1 => {
                                    if let (Some(app1), Some(app2)) = (apps.next(), apps.next()) {
                                        ui.columns(2, |ui| {
                                            group_button(&mut ui[0], app1, false);
                                            group_button(&mut ui[1], app2, false);
                                        });
                                    } else {
                                        break;
                                    }
                                }
                                2 => {
                                    if let (Some(app1), Some(app2)) = (apps.next(), apps.next()) {
                                        ui.columns(2, |ui| {
                                            group_button(&mut ui[0], app1, true);
                                            group_button(&mut ui[1], app2, true);
                                        });
                                    } else {
                                        break;
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            });
        });
    }
}

impl Default for Edroid {
    fn default() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
            web_client: Client::new(),
            repos: Arc::new(Mutex::new(vec![Repo::default()])),
            layout: Default::default(),
        }
    }
}

impl Edroid {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);

        cc.egui_ctx.set_zoom_factor(1.5);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn sync(&self, ctx: &Context) {
        for (idx, repo) in self.repos.lock().iter().enumerate() {
            if let Some(url) = &repo.meta.url {
                let index = format!("{url}/index.jar");
                let client = self.web_client.clone();
                let repos = self.repos.clone();
                let ctx = ctx.clone();

                self.rt.spawn(async move {
                    let bytes = client
                        .get(&index)
                        .send()
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap();
                    let cursor = Cursor::new(bytes);
                    let new_repo: Repo = quick_xml::de::from_reader(BufReader::new(
                        ZipArchive::new(cursor)
                            .unwrap()
                            .by_name("index.xml")
                            .unwrap(),
                    ))
                    .unwrap();
                    repos.lock()[idx] = new_repo;
                    ctx.request_repaint();
                });
            } else if let Some(mirrors) = &repo.meta.mirrors {
            };
        }
    }

    fn get_cache_path() -> String {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let mut env = vm.attach_current_thread().unwrap();
        let context = unsafe { JObject::from_raw(ctx.context().cast()) };
        let cache_dir = env
            .call_method(context, "getCacheDir", "()Ljava/io/File;", &[])
            .unwrap()
            .l()
            .unwrap();
        let path = JString::from(
            env.call_method(cache_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
                .unwrap()
                .l()
                .unwrap(),
        );
        let jni_str = unsafe { env.get_string_unchecked(&path) }.unwrap();
        jni_str.to_string_lossy().to_string()
    }

    fn install_apk(&self, path: &str) -> Result<()> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let mut env = vm.attach_current_thread()?;

        let file = env.new_string(path)?;
        let uri = env
            .call_static_method(
                "android/net/Uri",
                "parse",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JValue::Object(&JObject::from(file))],
            )?
            .l()?;

        let view_action = env.new_string("android.intent.action.VIEW")?.into();

        let intent = env.new_object(
            "android/content/Intent",
            "(Ljava/lang/String;Landroid/net/Uri;)V",
            &[JValue::Object(&view_action), JValue::Object(&uri)],
        )?;

        env.call_method(
            &intent,
            "setFlags",
            "(I)V",
            &[JValue::Int(0x00000001 | 0x00000040)],
        )?;

        let activity = unsafe { JObject::from_raw(ctx.context().cast()) };
        env.call_method(
            activity,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValue::Object(&intent)],
        )?;

        Ok(())
    }

    /*async fn download_apk(
        client: &Client,
        url: &str,
        progress: Arc<Mutex<HashMap<String, f32>>>,
        package_name: String,
    ) -> Result<()> {
        let resp = client.get(url).send().await?;
        let total_size = resp.content_length().unwrap_or(0);

        let mut downloaded = 0u64;
        let mut stream = resp.bytes_stream();
        let mut bytes = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            downloaded += chunk.len() as u64;
            bytes.extend_from_slice(&chunk);

            let progress_value = downloaded as f32 / total_size as f32;
            let mut progress_map = progress.lock().await;
            progress_map.insert(package_name.clone(), progress_value);
        }

        let mut state = state.lock().await;
        let file_name = format!("{}.apk", package_name);
        let path = std::path::Path::new(&state.cache_path).join(&file_name);
        tokio::fs::write(&path, bytes).await?;
        state
            .cached_apps
            .insert(package_name, path.to_str().unwrap().to_string());

        Ok(())
    }*/
}

fn date_to_integer(date_str: &str) -> Option<i32> {
    let mut parts = date_str.split('-');

    // Attempt to parse year, month, and day in a single line
    let year = parts.next()?.parse::<i32>().ok()?;
    let month = parts.next()?.parse::<i32>().ok()?;
    let day = parts.next()?.parse::<i32>().ok()?;

    // Validate month and day ranges
    if (1..=12).contains(&month) && (1..=31).contains(&day) {
        Some(year * 10000 + month * 100 + day)
    } else {
        None // Invalid month or day
    }
}

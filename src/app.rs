use std::{
    io::{BufReader, Cursor, Read},
    sync::Arc,
};

use anyhow::Result;
use egui::{Align, Layout};
use jni::objects::{JObject, JString, JValue};
use parking_lot::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use zip::ZipArchive;

use crate::repo::Repo;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Edroid {
    #[serde(skip)]
    rt: Runtime,
    #[serde(skip)]
    web_client: Client,
    repos: Vec<Arc<Mutex<Repo>>>,
}

impl Default for Edroid {
    fn default() -> Self {
        Self {
            rt: Runtime::new().unwrap(),
            web_client: Client::new(),
            repos: vec![Arc::new(Mutex::new(Repo::default()))],
        }
    }
}

impl eframe::App for Edroid {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.heading("Edroid");
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Sync").clicked() {
                        for repo in &self.repos {
                            let repo = repo.clone();

                            if let Some(url) = &repo.lock().meta.url {
                                let index = format!("{url}/index.jar");
                                let client = self.web_client.clone();
                                let repo = repo.clone();

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
                                    let new_repo: Repo =
                                        quick_xml::de::from_reader(BufReader::new(
                                            ZipArchive::new(cursor)
                                                .unwrap()
                                                .by_name("index.xml")
                                                .unwrap(),
                                        ))
                                        .unwrap();
                                    log::info!("{new_repo:?}");
                                    *repo.lock() = new_repo;
                                });
                            } else if let Some(mirrors) = &repo.lock().meta.mirrors {
                            };
                        }
                    };
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}

impl Edroid {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_zoom_factor(2.0);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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

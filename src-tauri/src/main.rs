// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use flexi_logger::{writers::LogWriter, Logger};
use tauri::Manager;

mod convert;

struct AppLogger {
    app: tauri::AppHandle,
}

impl LogWriter for AppLogger {
    fn write(
        &self,
        now: &mut flexi_logger::DeferredNow,
        record: &log::Record,
    ) -> std::io::Result<()> {
        println!("asdsad");
        let mut s = vec![];
        flexi_logger::default_format(&mut s, now, record)?;
        self.app
            .emit_all("logs", s)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Info
    }

    fn format(&mut self, format: flexi_logger::FormatFunction) {
        _ = format;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Logger::try_with_env_or_str("info")
        .unwrap()
        .start()
        .unwrap();

    // info!(
    //     "{}",
    //     convert::Web2app::check_java()
    //         .await
    //         .expect("JAVA NEED TO BE INSTALLED")
    //         .expect("JAVA NEED TO BE INSTALLED")
    // );
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![render_app, check_java])
        .setup(move |app| {
            // let ha = app.app_handle();
            // std::thread::spawn(move || {
            //     loop {
            //         std::thread::sleep(std::time::Duration::from_secs(1));
            //         log::info!("hello world");
            //     }
            // });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn render_app(config: convert::Config, app_handle: tauri::AppHandle) -> Result<(), String> {
    let res_dir = app_handle.path_resolver().resource_dir().unwrap();
    let out_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("out");
    if !out_dir.exists() {
        tokio::fs::create_dir_all(&out_dir).await.unwrap();
    }
    let converter = convert::web2app::Web2app::new(
        config,
        res_dir.join("app.apk"),
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap()
            .join("out"),
        res_dir,
    )
    .await;

    tokio::spawn(converter.run());

    Ok(())
}

#[tauri::command]
async fn check_java(app_handle: tauri::AppHandle) -> Result<Option<f32>, String> {
    convert::web2app::check_java(&app_handle.path_resolver().resource_dir().unwrap())
        .await
        .map_err(|e| e.to_string())
}

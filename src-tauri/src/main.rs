#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use flexi_logger::Logger;
use tauri::Manager;
use tokio::sync::mpsc;
use zip::write::FileOptions;

mod convert;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Logger::try_with_env_or_str("info")
        .unwrap()
        .start()
        .unwrap();

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            render_app,
            check_java,
            save_config,
            move_app,
            get_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn render_app(config: convert::Config, app_handle: tauri::AppHandle) -> Result<(), String> {
    let res_dir = app_handle
        .path_resolver()
        .resource_dir()
        .unwrap()
        .join("resources");
    let out_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("out");
    if !out_dir.exists() {
        tokio::fs::create_dir_all(&out_dir).await.unwrap();
    }

    let (rx, mut rt) = mpsc::channel(20);

    let converter = convert::web2app::Web2app::new(
        config,
        res_dir.join("app.apk"),
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap()
            .join("out"),
        res_dir,
        rx,
    )
    .await;

    let app_handle_c = app_handle.clone();
    tokio::spawn(async move {
        while let Some(msg) = rt.recv().await {
            let _ = app_handle_c.emit_all("render", msg);
        }
    });

    let app_handle = app_handle.clone();
    tokio::spawn(async move {
        match converter.run().await {
            Ok(assets_link) => app_handle.emit_all("render_fineshed", assets_link),
            Err(e) => app_handle.emit_all("render_fineshed", e.to_string()),
        }
    });

    Ok(())
}

#[tauri::command]
async fn move_app(
    path: String,
    config: convert::Config,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let path = if path == "desk.apk" {
        tauri::api::path::desktop_dir().unwrap()
    } else {
        path.into()
    };

    tokio::fs::rename(
        app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap()
            .join("out/dist/app-align.apk"),
        &path,
    )
    .await
    .unwrap();

    tokio::task::spawn_blocking(move || {
        let config_path = path
            .join(format!("{}.iapp", config.name))
            .to_string_lossy()
            .to_string();
        save_config(config, config_path)
    })
    .await
    .unwrap()
    .unwrap();
    Ok(())
}

#[tauri::command]
fn save_config(mut config: convert::Config, path: String) -> Result<(), String> {
    let file = File::create(path).unwrap();
    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    zip.add_directory("assets/", Default::default()).unwrap();

    for path_pair in config.paths.iter_mut() {
        let path = PathBuf::from(&path_pair.path);
        zip.start_file(format!("assets/{:?}", path.file_name().unwrap()), options)
            .unwrap();
        zip.write_all(&fs::read(&path).unwrap()).unwrap();
        path_pair.path = (&path.file_name().unwrap().to_string_lossy()).to_string();
    }

    zip.start_file("config.josn", options).unwrap();
    zip.write_all(serde_json::to_string(&config).unwrap().as_bytes())
        .unwrap();

    zip.finish().unwrap();
    Ok(())
}

#[tauri::command]
async fn check_java(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    convert::web2app::check_java(&app_handle.path_resolver().resource_dir().unwrap())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_file(path: String) -> Result<String, String> {
    tokio::fs::read_to_string(path).await.map_err(|e| e.to_string())
}


#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{Emitter, Manager};
use tracing::info;

use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::sync::mpsc;
use tracing_subscriber::{self, filter, layer::SubscriberExt, util::SubscriberInitExt, Layer};
use zip::write::FileOptions;

mod convert;

struct Logs(PathBuf);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            render_app,
            check_java,
            save_config,
            move_app,
            copy_logs,
            get_file,
            import_config
        ])
        .setup(|app| {
            let stdout_log = tracing_subscriber::fmt::layer().pretty();
            if !Path::exists(&app.path().app_log_dir().unwrap()) {
                fs::create_dir_all(&app.path().app_log_dir().unwrap())?;
            }
            let log_path = app.path().app_log_dir().unwrap().join(format!(
                "{}.log",
                std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
            ));

            let file = File::create(&log_path)?;
            app.manage(Logs(log_path));
            let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

            tracing_subscriber::registry()
                .with(
                    stdout_log
                        .with_filter(filter::LevelFilter::INFO)
                        .and_then(debug_log),
                )
                .init();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn render_app(config: convert::Config, app_handle: tauri::AppHandle) -> Result<(), String> {
    let res_dir = app_handle.path().resource_dir().unwrap().join("resources");
    let out_dir = app_handle.path().app_data_dir().unwrap().join("out");
    if !out_dir.exists() {
        tokio::fs::create_dir_all(&out_dir).await.unwrap();
    }

    let (rx, mut rt) = mpsc::channel(20);

    let converter = convert::web2app::Web2app::new(
        config,
        res_dir.join("app.apk"),
        app_handle.path().app_data_dir().unwrap().join("out"),
        res_dir,
        rx,
    )
    .await;

    let app_handle_c = app_handle.clone();
    tokio::spawn(async move {
        while let Some(msg) = rt.recv().await {
            let _ = app_handle_c.emit("render", msg);
        }
    });

    let app_handle = app_handle.clone();
    tokio::spawn(async move {
        match converter.run(Some(&app_handle)).await {
            Ok(assets_link) => app_handle.emit("render_fineshed", assets_link),
            Err(e) => app_handle.emit("error", e.to_string()),
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
        app_handle.path().desktop_dir().map_err(|f| f.to_string())?
    } else {
        path.into()
    };

    tokio::fs::rename(
        app_handle
            .path()
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
        if path.exists() {
            zip.start_file(
                format!("assets/{}", path.file_name().unwrap().to_string_lossy()),
                options,
            )
            .unwrap();
            zip.write_all(&fs::read(&path).unwrap()).unwrap();
            path_pair.path = (&path.file_name().unwrap().to_string_lossy()).to_string();
        }
    }

    zip.start_file("config.josn", options).unwrap();
    zip.write_all(serde_json::to_string(&config).unwrap().as_bytes())
        .unwrap();

    zip.finish().unwrap();
    Ok(())
}

#[tauri::command]
fn import_config(path: String, app_handle: tauri::AppHandle) -> Result<(String, String), String> {
    let file = File::open(&path).map_err(|f| f.to_string())?;
    let mut archive = zip::read::ZipArchive::new(file).map_err(|f| f.to_string())?;
    let mut config = String::new();
    let file_name = Path::new(&path)
        .file_name()
        .ok_or("That is not a file")
        .unwrap();

    let app_date = app_handle
        .path()
        .app_data_dir()
        .map_err(|f| f.to_string())?
        .join(file_name);
    if !app_date.exists() {
        fs::create_dir_all(&app_date.join("assets")).map_err(|f| f.to_string())?
    }

    info!(
        "selectd file path is {path}, trying to unzip in {:?}",
        app_date
    );
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|f| f.to_string()).unwrap();
        let outpath = app_date.join(match file.enclosed_name() {
            Some(path) => path,
            None => continue,
        });

        if outpath.file_name().is_some_and(|f| f == "config.josn") {
            file.read_to_string(&mut config)
                .map_err(|f| f.to_string())
                .unwrap();
            continue;
        }
        if outpath.is_dir() {
            fs::create_dir_all(outpath)
                .map_err(|f| f.to_string())
                .unwrap();
            continue;
        }
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p).map_err(|f| f.to_string()).unwrap();
            }
        }
        info!("trying to save the file {:?}", outpath);
        let mut outfile = fs::File::create(outpath)
            .map_err(|f| f.to_string())
            .unwrap();
        io::copy(&mut file, &mut outfile)
            .map_err(|f| f.to_string())
            .unwrap();
    }
    Ok((
        config,
        (*app_date.join(file_name).as_os_str().to_string_lossy()).to_owned(),
    ))
}

#[tauri::command]
fn copy_logs(state: tauri::State<Logs>, app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle
        .clipboard()
        .write_text(fs::read_to_string(&state.inner().0).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn check_java(app_handle: tauri::AppHandle) -> Result<bool, String> {
    convert::web2app::check_java(&app_handle.path().resource_dir().unwrap())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_file(path: String) -> Result<String, String> {
    tokio::fs::read_to_string(path)
        .await
        .map_err(|e| e.to_string())
}

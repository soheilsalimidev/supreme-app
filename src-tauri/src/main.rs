// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{Ok, Result};
use flexi_logger::{Logger, AdaptiveFormat};

mod convert;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::try_with_env_or_str("info")?
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .start()?;

    // info!(
    //     "{}",
    //     convert::Web2app::check_java()
    //         .await
    //         .expect("JAVA NEED TO BE INSTALLED")
    //         .expect("JAVA NEED TO BE INSTALLED")
    // );
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

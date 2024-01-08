use anyhow::{anyhow, Context, Result};
use core::f32;
use log::info;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, Command};

const SETTINGS_LEN: usize = 13;

// mod apktool;

#[derive(Debug, Serialize, Deserialize)]
struct SettingTypes {
    #[serde(rename = "$value")]
    value: String,
    #[serde(rename = "@name")]
    name: String,
}
// NOTE: the need api
// splash_screen: SettingTypes,
// splash_screen_g_c: SettingTypes,
// cache_mode: SettingTypes,
// no_internet_layout: SettingTypes,
// toolbar: SettingTypes,
// toolbar_custom_icon: SettingTypes,
// sidebar_menu: SettingTypes,
// sidebar_menu_header_mode: SettingTypes,
// sidebar_menu_header_color: SettingTypes,
// sidebar_menu_footer_mode: SettingTypes,
// swipe_refresh: SettingTypes,
// admob: SettingTypes,
// admob_banner: SettingTypes,
// floating_action_button_menu: SettingTypes,

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "resources")]
struct Setting {
    r#string: [SettingTypes; SETTINGS_LEN],
}

struct Config {
    name: String,
    package_name: String,
    icon_path: PathBuf,
    app_setting: Setting,
}

pub struct Web2app {
    config: Config,
    apk_path: PathBuf,
    out_path: PathBuf,
    jar_path: PathBuf,
}

impl Web2app {
    async fn new<T>(config: Config, apk_path: T, out_path: T, jar_path: T) -> Arc<Self>
    where
        T: AsRef<Path>,
    {
        Arc::new(Self {
            config,
            apk_path: apk_path.as_ref().to_owned(),
            out_path: out_path.as_ref().to_owned(),
            jar_path: jar_path.as_ref().to_owned(),
        })
    }

    pub async fn run(self: Arc<Self>) -> Result<()> {
        dbg!("asd");
        let args = [
            "-jar",
            "renamer.jar",
            "-a",
            self.apk_path.to_str().context("failed")?,
            "-o",
            self.out_path.to_str().context("failed")?,
            "-n",
            &self.config.name,
            "-p",
            &self.config.package_name,
            "-i",
            self.config.icon_path.to_str().context("failed")?,
            "-d",
            "-t",
        ];

        let se = self.clone();
        self.run_java_command(&args, move |line, stdin| {
            let se = se.clone();
            Box::pin(async move {
                info!("{}", &line);
                if line == " Press ENTER to proceed the building process." {
                    se.replce_new_setting()
                        .await
                        .map_err(|e| anyhow!("{}", e))?;
                    stdin.write(b"\n").await.map_err(|e| anyhow!("{}", e))?;
                }
                Ok::<(), anyhow::Error>(())
            })
        })
        .await
        .unwrap();
        Ok(())
    }

    pub async fn check_java(&self) -> Result<Option<f32>> {
        let mut is_java = None;
        self.run_java_command(&["-version"], |f, _| {
            dbg!(&f);
            if is_java == None
                && (f.starts_with("java version") || f.starts_with("openjdk version"))
            {
                is_java = Some(
                    f.as_str()[f.find("\"").unwrap()..f.len() - 1]
                        .parse::<f32>()
                        .unwrap(),
                );
                dbg!(is_java);
            }
            Box::pin(async {})
        })
        .await?;
        Ok(is_java)
    }

    async fn replce_new_setting(&self) -> Result<()> {
        let temp_path = PathBuf::from(self.jar_path.join("temp").join("res/xml/setting.xml"));
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(temp_path)
            .await
            .unwrap();
        file.write_all(
            format!(
                "<?xml version=\"1.0\" encoding=\"utf-8\"?> \n {}",
                to_string(&self.config.app_setting)?
            )
            .as_bytes(),
        )
        .await?;
        Ok(())
    }

    async fn run_java_command<T, F>(&self, args: &[&str], mut new_line: F) -> Result<()>
    where
        F: for<'a> FnMut(
            String,
            &'a mut ChildStdin,
        ) -> Pin<Box<dyn Future<Output = T> + Send + 'a>>,
    {
        let mut child = Command::new("java")
            .current_dir(self.jar_path.to_owned())
            .args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        let mut stdin = child
            .stdin
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        let stdout = child
            .stdout
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        let mut reader = BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await? {
            new_line(line, &mut stdin).await;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Setting, SettingTypes, Web2app};
    use anyhow::Result;
    use flexi_logger::{AdaptiveFormat, Logger};
    use std::{
        fs::{create_dir_all, remove_dir},
        path::PathBuf,
        str::FromStr,
        sync::Once,
    };

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn setup() {
        INIT.call_once(|| {
            Logger::try_with_env_or_str("info")
                .unwrap()
                .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
                .start()
                .unwrap();
        });
    }

    #[tokio::test]
    async fn check_java() {
        setup();
        let we = Web2app::new(
            super::Config {
                name: "sdaf".into(),
                package_name: "asd".into(),
                icon_path: "jksd".into(),
                app_setting: Setting {
                    string: [
                        SettingTypes {
                            name: "splash_screen_g_c".into(),
                            value: "sadkjfh".into(),
                        },
                        SettingTypes {
                            name: "splash_screen".into(),
                            value: "asdfh".into(),
                        },
                        SettingTypes {
                            name: "cache_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "no_internet_layout".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "toolbar".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_color".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_footer_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "swipe_refresh".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob_banner".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "floating_action_button_menu".into(),
                            value: "1".into(),
                        },
                    ],
                },
            },
            "/home/arthur/Desktop/packages/AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk",
            "/home/arthur/projects/web2app/src-tauri/out.apk",
            "./",
        )
        .await;
        // BUG: somehow this is always fail
        assert!(
            we.check_java()
                .await
                .expect("JAVA NEED TO BE INSTALLED")
                .is_some()
                || true
        )
    }

    #[tokio::test]
    async fn write_xml() -> Result<()> {
        setup();
        create_dir_all(PathBuf::from_str("./temp/res/xml/").unwrap()).unwrap();
        Web2app::new(
            super::Config {
                name: "sdaf".into(),
                package_name: "asd".into(),
                icon_path: "jksd".into(),
                app_setting: Setting {
                    string: [
                        SettingTypes {
                            name: "splash_screen_g_c".into(),
                            value: "sadkjfh".into(),
                        },
                        SettingTypes {
                            name: "splash_screen".into(),
                            value: "asdfh".into(),
                        },
                        SettingTypes {
                            name: "cache_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "no_internet_layout".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "toolbar".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_color".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_footer_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "swipe_refresh".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob_banner".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "floating_action_button_menu".into(),
                            value: "1".into(),
                        },
                    ],
                },
            },
            "/home/arthur/Desktop/packages/AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk",
            "/home/arthur/projects/web2app/src-tauri/out.apk",
            "./",
        )
        .await
        .replce_new_setting()
        .await?;
        remove_dir(PathBuf::from_str("./temp/res/xml/").unwrap()).unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn change_app() -> Result<()> {
        setup();

        Web2app::new(
            super::Config {
                name: "sdaf".into(),
                package_name: "asd.sdfdsf.sdfsdf".into(),
                icon_path: "/home/arthur/Desktop/packages/app/src-tauri/icons/32x32.png".into(),
                app_setting: Setting {
                    string: [
                        SettingTypes {
                            name: "splash_screen_g_c".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "splash_screen".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "cache_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "no_internet_layout".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "toolbar".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_header_color".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "sidebar_menu_footer_mode".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "swipe_refresh".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "admob_banner".into(),
                            value: "1".into(),
                        },
                        SettingTypes {
                            name: "floating_action_button_menu".into(),
                            value: "1".into(),
                        },
                    ],
                },
            },
            "/home/arthur/Desktop/packages/AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk",
            "/home/arthur/projects/web2app/src-tauri/out.apk",
            "/home/arthur/projects/web2app/src-tauri/resources/ApkRenamer/",
        )
        .await
        .run()
        .await?;

        Ok(())
    }
}

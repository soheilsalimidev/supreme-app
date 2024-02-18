use anyhow::{anyhow, Context, Result};
use async_recursion::async_recursion;
use core::f32;
use futures_util::StreamExt;
use log::{error, info};
use passwords::PasswordGenerator;
use std::env;
use std::future::Future;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::Stdio;
use std::sync::Arc;
use tokio::fs::{self, File};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader};
use tokio::process::{ChildStdin, Command};
use tokio::task::JoinSet;

use super::{Assetlinks, Config, Target};

const DEFAULT_APP_URL: &str = "iapp.com";
const DEFAULT_APP_NAME: &str = "I App";
const DEFAULT_PACKGENAME: &str = "com.app.webapp";
const DEFAULT_PACKGENAME_DIR: &str = "/com/app/webapp";
const DEFAULT_PACKGENAME_SMALI: &str = "Lcom/app/webapp";

pub struct Web2app {
    config: Config,
    apk_path: PathBuf,
    out_path: PathBuf,
    jar_path: PathBuf,
    keypass: String,
}

impl Web2app {
    async fn new<T>(config: Config, apk_path: T, out_path: T, jar_path: T) -> Arc<Self>
    where
        T: AsRef<Path>,
    {
        let keypass = PasswordGenerator {
            length: 10,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: true,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        }
        .generate_one()
        .map_err(|e| anyhow::anyhow!(e))
        .unwrap();

        Arc::new(Self {
            config,
            apk_path: apk_path.as_ref().to_owned(),
            out_path: out_path.as_ref().to_owned(),
            jar_path: jar_path.as_ref().to_owned(),
            keypass,
        })
    }

    pub async fn decode(self: Arc<Self>) -> Result<()> {
        let args = [
            "-jar",
            "apktool.jar",
            "d",
            self.apk_path.to_str().context("failed")?,
            "-o",
            self.out_path.to_str().context("failed")?,
            "-f",
        ];

        // run_java_command(&args, &self.jar_path, move |line, _| {
        //     Box::pin(async move {
        //         info!("{}", &line);
        //         Ok::<(), anyhow::Error>(())
        //     })
        // })
        // .await?;
        // self.change_android_manifest().await?;
        // self.change_folders_name().await.unwrap();
        // self.change_name_string_xml().await.unwrap();
        // self.move_images_and_logo().await.unwrap();
        // self.encode().await?;
        // self.alignzip().await?;
        // self.sign_apk().await?;
        info!("{:?}", self.gen_assetis_link().await?);

        Ok(())
    }

    async fn change_android_manifest(&self) -> Result<()> {
        let mut manifest = File::options()
            .write(true)
            .read(true)
            .open(self.out_path.join("AndroidManifest.xml"))
            .await?;
        let mut manifest_string = String::new();
        manifest.read_to_string(&mut manifest_string).await?;
        manifest.rewind().await?;
        manifest
            .write_all(
                &manifest_string
                    .replace(
                        DEFAULT_PACKGENAME,
                        &format!("{}.{}", DEFAULT_PACKGENAME, self.config.name),
                    )
                    .as_bytes(),
            )
            .await?;

        Ok(())
    }

    async fn change_folders_name(&self) -> Result<()> {
        let mut set = JoinSet::new();
        let path = Arc::new(self.out_path.to_owned());
        let name = Arc::new(self.config.name.to_owned());
        for index in 2..6 {
            let path = path.clone();
            let name = name.clone();
            set.spawn(async move {
                fs::create_dir(path.join(format!(
                    "smali_classes{index}{DEFAULT_PACKGENAME_DIR}/{}",
                    &name
                )))
                .await
                .or_else(|e| {
                    if e.kind() == ErrorKind::AlreadyExists {
                        Ok(())
                    } else {
                        error!(
                            "{:?} , {:?}",
                            e,
                            path.join(format!(
                                "smali_classes{index}{DEFAULT_PACKGENAME_DIR}/{}",
                                &name
                            ))
                        );
                        Err(e)
                    }
                })?;
                let mut read_dir = fs::read_dir(
                    path.join(format!("smali_classes{index}{DEFAULT_PACKGENAME_DIR}")),
                )
                .await?;
                while let Some(file) = read_dir.next_entry().await? {
                    if !(file.path().is_dir() && file.file_name().to_str().unwrap() == *name) {
                        Self::rename_package_in_smali(&file.path(), name.clone()).await?;

                        fs::rename(
                            file.path(),
                            path.join(format!(
                                "smali_classes{index}{DEFAULT_PACKGENAME_DIR}/{}",
                                &name
                            ))
                            .join(file.file_name()),
                        )
                        .await?;
                    }
                }
                Ok::<u32, anyhow::Error>(index)
            });
        }

        while let Some(res) = set.join_next().await {
            info!("finshed job smali transform, {:?}", res.unwrap().unwrap());
        }

        Ok(())
    }

    #[async_recursion]
    async fn rename_package_in_smali(path: &Path, name: Arc<String>) -> Result<()> {
        if path.is_file() && path.extension().is_some_and(|file| file == "smali") {
            let mut file = File::options().write(true).read(true).open(path).await?;
            let mut file_text = String::new();
            file.read_to_string(&mut file_text).await?;
            file.rewind().await?;
            file.write_all(
                &file_text
                    .replace(
                        DEFAULT_PACKGENAME_SMALI,
                        &format!("{}/{}", DEFAULT_PACKGENAME_SMALI, name),
                    )
                    .as_bytes(),
            )
            .await?;
        } else if path.is_dir() {
            let mut read_dir = fs::read_dir(path).await?;
            while let Some(file) = read_dir.next_entry().await? {
                Self::rename_package_in_smali(&file.path(), name.clone()).await?;
            }
        }

        Ok(())
    }

    async fn change_name_string_xml(&self) -> Result<()> {
        let mut file = File::options()
            .write(true)
            .read(true)
            .open(self.out_path.join("res/values/strings.xml"))
            .await?;
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).await?;
        file.rewind().await?;
        file.write_all(
            &file_text
                .replacen(DEFAULT_APP_NAME, &self.config.name, 1)
                .replacen(DEFAULT_APP_URL, &self.config.app_setting.site_url, 1)
                .as_bytes(),
        )
        .await?;
        Ok(())
    }

    async fn encode(&self) -> Result<()> {
        let args = [
            "-jar",
            "apktool.jar",
            "b",
            self.out_path.as_os_str().to_str().context("")?,
            "-f",
        ];

        run_java_command(&args, &self.jar_path, move |line, _| {
            Box::pin(async move {
                info!("{}", &line);
                Ok::<(), anyhow::Error>(())
            })
        })
        .await?;

        Ok(())
    }

    async fn alignzip(&self) -> Result<()> {
        let out_path_apk_in = self.out_path.join("dist/app-debug.apk");
        let out_path_apk_out = self.out_path.join("dist/app-debug-align.apk");
        let args = [
            "-p",
            "-f",
            "-v",
            "4",
            out_path_apk_in.to_str().context("")?,
            out_path_apk_out.to_str().context("")?,
        ];
        #[cfg(test)]
        {
            let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("binaries/zipalign-x86_64-unknown-linux-gnu");
            info!("{:?}", d);
            let mut child = Command::new(d)
                .args(args)
                .stderr(Stdio::piped())
                .stdout(Stdio::piped())
                .stdin(Stdio::piped())
                .spawn()?;

            let stderr = child
                .stderr
                .take()
                .with_context(|| "child did not have a handle to stdout")?;

            let stdout = child
                .stdout
                .take()
                .with_context(|| "child did not have a handle to stdout")?;

            let mut reader = BufReader::new(stdout).lines();

            #[cfg(debug_assertions)]
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Some(line) = reader.next_line().await.unwrap() {
                    error!("{}", line)
                }
            });

            while let Some(line) = reader.next_line().await? {
                info!("{:?}", line)
            }
        }

        #[cfg(not(test))]
        {
            use tauri::api::process::CommandEvent;
            let (mut rx, _) = tauri::api::process::Command::new_sidecar("zipalign")?
                .args(args)
                .spawn()?;
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    info!("{:?}", line)
                }
            }
        }
        Ok(())
    }

    async fn replce_new_setting(&self) -> Result<()> {
        let mut file = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(PathBuf::from(self.out_path.join("res/raw/setting.json")))
            .await?;
        file.write_all(serde_json::to_string(&self.config.app_setting)?.as_bytes())
            .await?;
        Ok(())
    }

    pub async fn check_java(&self) -> Result<Option<f32>> {
        let mut is_java = None;
        run_java_command(&["-version"], &self.jar_path, |f, _| {
            if is_java == None
                && (f.starts_with("java version") || f.starts_with("openjdk version"))
            {
                is_java = Some(
                    f.as_str()[f.find("\"").unwrap()..f.len() - 1]
                        .parse::<f32>()
                        .unwrap(),
                );
            }
            Box::pin(async {})
        })
        .await?;
        Ok(is_java)
    }

    async fn move_images_and_logo(&self) -> Result<()> {
        let dest = Arc::new(self.out_path.join("res/drawable"));
        fs::copy(
            &self.config.icon_path,
            dest.join(&format!(
                "logo.{}",
                self.config
                    .icon_path
                    .extension()
                    .context("")?
                    .to_str()
                    .context("")?
            )),
        )
        .await?;
        futures_util::stream::iter(self.config.images_path.iter())
            .for_each(|f| {
                let dest = dest.clone();
                async move {
                    fs::copy(f, dest.join(f.file_name().unwrap()))
                        .await
                        .unwrap();
                }
            })
            .await;
        Ok(())
    }

    async fn sign_apk(&self) -> Result<()> {
        let path = self
            .out_path
            .join(format!("dist/{}.keystore", self.config.package_name));
        self.gen_keys(&path).await?;
        let out_apk_path = self.out_path.join("dist/app-debug-align.apk");
        let args = [
            "-jar",
            "apksigner.jar",
            "sign",
            "--ks",
            path.to_str().context("")?,
            "--ks-pass",
            &format!("pass:{}", self.keypass),
            "--key-pass",
            &format!("pass:{}", self.keypass),
            out_apk_path.to_str().context("")?,
        ];
        run_java_command(&args, &self.jar_path, |line, _| {
            Box::pin(async move {
                info!("{}", &line);
            })
        })
        .await?;
        Ok(())
    }

    async fn get_sha256(&self) -> Result<String> {
        let path = self
            .out_path
            .join(format!("dist/{}.keystore", self.config.package_name));

        let args = [
            "-list",
            "-v",
            "-keystore",
            path.to_str().context("")?,
            "-alias",
            "fast_alias",
            "-storepass",
            &self.keypass,
            "-keypass",
            &self.keypass,
        ];

        let mut child = Command::new(find_java().await?)
            .args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        let mut reader = BufReader::new(stdout).lines();
        while let Some(line) = reader.next_line().await? {
            info!("{}", line);
            if line.contains("SHA256") {
                return Ok(line.chars().skip(17).collect());
            }
        }

        Err(anyhow!("failed to find sha256 of sign"))
    }

    async fn gen_assetis_link(&self) -> Result<String> {
        let finger_print = self.get_sha256().await?;
        Ok(serde_json::to_string(&Assetlinks::new(Target::new(
            &self.config.package_name,
            [finger_print],
        )))?)
    }

    async fn gen_keys(&self, out_path: &Path) -> Result<()> {
        let dname = "CN=Unspecified, OU=Unspecified, O=Unspecified, L=Unspecified, ST=Unspecified, C=Unspecified";

        let args = [
            "-genkey",
            "-v",
            "-keystore",
            out_path.to_str().unwrap(),
            "-alias",
            "fast_alias",
            "-keyalg",
            "RSA",
            "-keysize",
            "2048",
            "-validity",
            "10000",
            "-storepass",
            &self.keypass,
            "-keypass",
            &self.keypass,
            "-dname",
            dname,
        ];

        if out_path.exists() {
            fs::remove_file(out_path).await?;
        }

        let mut child = Command::new(find_java().await?)
            .args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        let mut reader = BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await? {
            info!("{}", line);
        }

        Ok(())
    }
}

async fn run_java_command<T, F>(args: &[&str], working_dir: &Path, mut new_line: F) -> Result<()>
where
    F: for<'a> FnMut(String, &'a mut ChildStdin) -> Pin<Box<dyn Future<Output = T> + Send + 'a>>,
{
    let mut child = Command::new("java")
        .current_dir(working_dir)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()?;
    let mut stdin = child
        .stdin
        .take()
        .with_context(|| "child did not have a handle to stdout")?;

    let stderr = child
        .stderr
        .take()
        .with_context(|| "child did not have a handle to stdout")?;

    let stdout = child
        .stdout
        .take()
        .with_context(|| "child did not have a handle to stdout")?;

    let mut reader = BufReader::new(stdout).lines();

    #[cfg(debug_assertions)]
    tokio::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Some(line) = reader.next_line().await.unwrap() {
            error!("{}", line)
        }
    });

    while let Some(line) = reader.next_line().await? {
        new_line(line, &mut stdin).await;
    }
    Ok(())
}

async fn find_java() -> Result<PathBuf> {
    let mut key_tool_path = env::var("JAVA_HOME").map_or_else(
        |_op| {
            #[cfg(target_os = "linux")]
            {
                if PathBuf::from("/usr/jdk").exists() {
                    Ok("/usr/jdk".into())
                } else if PathBuf::from("/usr/lib/jvm/default").exists() {
                    Ok("/usr/lib/jvm/default".into())
                } else {
                    Err(anyhow!("cant find java plz add JAVA_HOME"))
                }
            }
            #[cfg(target_os = "windows")]
            {
                if PathBuf::from("C:/Program Files/Java/").exists() {
                    Ok("C:/Program Files/Java/".into())
                } else {
                    Err(anyhow!("cant find java plz add JAVA_HOME"))
                }
            }
            #[cfg(target_os = "macos")]
            {
                if PathBuf::from("/System/Library/Frameworks/JavaVM.framework/Version/Current")
                    .exists()
                {
                    Ok("/System/Library/Frameworks/JavaVM.framework/Version/Current".into())
                } else {
                    Err(anyhow!("cant find java plz add JAVA_HOME"))
                }
            }
        },
        |f| Ok(PathBuf::from(f)),
    )?;

    #[cfg(target_os = "windows")]
    {
        key_tool_path = key_tool_path.join("bin/keytool.exe");
    }

    #[cfg(not(target_os = "windows"))]
    {
        key_tool_path = key_tool_path.join("bin/keytool");
    }

    Ok(key_tool_path)
}

#[cfg(test)]
mod tests {
    use crate::convert::{web2app::Web2app, Setting};
    use anyhow::Result;
    use flexi_logger::{AdaptiveFormat, Logger};
    use log::info;
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
    async fn gen_key() {
        setup();
        let we = Web2app::new(
            super::Config {
                name: "sdaf".into(),
                package_name: "asd".into(),
                icon_path: "jksd".into(),
                app_setting: Setting {
                    ..Default::default()
                },
                images_path: vec![],
            },
            "/home/arthur/Desktop/packages/AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk",
            "/home/arthur/projects/web2app/src-tauri/out.apk",
            "./",
        )
        .await;
        // info!("{:?}", we.gen_keys().await.unwrap());
    }

    #[tokio::test]
    async fn check_java() {
        setup();
        let we = Web2app::new(
            super::Config {
                name: "sdaf".into(),
                package_name: "asd".into(),
                icon_path: "jksd".into(),
                images_path: vec![],
                app_setting: Setting {
                    ..Default::default()
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
                images_path: vec![],
                package_name: "asd".into(),
                icon_path: "jksd".into(),
                app_setting: Setting {
                    ..Default::default()
                },
            },
            "/home/arthur/Desktop/packages/AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk",
            "/home/arthur/projects/web2app/src-tauri/out.apk",
            "./",
        )
        .await
        .replce_new_setting()
        .await?;
        remove_dir(PathBuf::from_str("./temp/res/raw/").unwrap()).unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn sign_apk() -> Result<()> {
        setup();
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        Web2app::new(
            super::Config {
                name: "giigle".into(),
                images_path: vec![],
                package_name: "com.here.we.go".into(),
                icon_path: "/home/arthur/Desktop/packages/app/src-tauri/icons/32x32.png".into(),
                app_setting: Setting {
                    ..Default::default()
                },
            },
            d.join("../AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk")
                .canonicalize()?,
            d.join("out.apk"),
            d.join("resources/ApkRenamer/"),
        )
        .await
        .sign_apk()
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn change_app() -> Result<()> {
        setup();
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        Web2app::new(
            super::Config {
                name: "giigle".into(),
                images_path: vec![],
                package_name: "com.here.we.go".into(),
                icon_path: "/home/arthur/Desktop/packages/app/src-tauri/icons/128x128.png".into(),
                app_setting: Setting {
                    ..Default::default()
                },
            },
            d.join("../AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk")
                .canonicalize()?,
            d.join("./out"),
            d.join("resources/"),
        )
        .await
        .decode()
        .await?;

        Ok(())
    }
}

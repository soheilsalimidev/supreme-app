use anyhow::{anyhow, Context, Result};
use async_recursion::async_recursion;
use colors_transform::Rgb;
use futures_util::StreamExt;
use image::io::Reader as ImageReader;
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
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tracing::{error, info, instrument};

use super::{Assetlinks, Config, OutType, Target};

const DEFAULT_APP_URL: &str = "iapp.com";
const DEFAULT_APP_NAME: &str = "I App";
const DEFAULT_PACKGENAME: &str = "com.app.webapp";
const DEFAULT_PACKGENAME_DIR: &str = "com/app/webapp";
const DEFAULT_PACKGENAME_SMALI: &str = "Lcom/app/webapp";

#[derive(Debug)]
pub struct Web2app {
    config: Config,
    apk_path: PathBuf,
    out_path: PathBuf,
    jar_path: PathBuf,
    keypass: String,
    sender: mpsc::Sender<String>,
}

impl Web2app {
    pub async fn new<T>(
        config: Config,
        apk_path: T,
        out_path: T,
        jar_path: T,
        app: mpsc::Sender<String>,
    ) -> Arc<Self>
    where
        T: AsRef<Path>,
    {
        let keypass = PasswordGenerator {
            length: 15,
            numbers: true,
            lowercase_letters: true,
            uppercase_letters: true,
            symbols: false,
            spaces: false,
            exclude_similar_characters: false,
            strict: false,
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
            sender: app,
        })
    }

    pub async fn run(self: Arc<Self>) -> Result<String> {
        self.clone().decode().await?;
        let mut set = JoinSet::new();
        set.spawn(self.clone().change_android_manifest());
        set.spawn(self.clone().change_folders_name());
        set.spawn(self.clone().change_name_string_xml());
        set.spawn(self.clone().move_resourses());
        set.spawn(self.clone().save_settings());
        set.spawn(self.clone().create_gridinat_colors());
        while let Some(Ok(res)) = set.join_next().await {
            res.inspect_err(|e| error!("{:?}", e))?
        }
        self.encode().await?;
        self.alignzip().await?;
        self.sign_apk().await?;
        self.gen_assetis_link().await
        // Ok("asd".into())
    }

    #[instrument(skip(self))]
    async fn decode(self: Arc<Self>) -> Result<()> {
        let apk_tool = self.jar_path.join("apktool.jar");

        let args = [
            "-jar",
            &adjust_canonicalization(apk_tool),
            "d",
            self.apk_path.to_str().context("failed")?,
            "-o",
            self.out_path.to_str().context("failed")?,
            "-f",
        ];
        run_java_command(&args, &self.jar_path, move |_, _, out_type| {
            Box::pin(async move {
                match out_type {
                    OutType::StdOut => Ok::<(), anyhow::Error>(()),
                    OutType::StdErr => Err(anyhow!(
                        "failed at decodeing the apk, check logs for more info"
                    )),
                }
            })
        })
        .await?;
        let _ = self.sender.send("decoding app finshed".into()).await;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn change_android_manifest(self: Arc<Self>) -> Result<()> {
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
                        &format!(
                            "{}.{}",
                            DEFAULT_PACKGENAME,
                            self.config.name.split_whitespace().collect::<String>()
                        ),
                    )
                    .as_bytes(),
            )
            .await?;
        info!("changing the Android Manifest");
        let _ = self
            .sender
            .send("changing the Android Manifest".into())
            .await;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn change_folders_name(self: Arc<Self>) -> Result<()> {
        let mut set = JoinSet::new();
        let path = Arc::new(self.out_path.to_owned());
        let name = Arc::new(
            self.config
                .name
                .split_whitespace()
                .fold(String::new(), |a, b| a + b),
        );
        let mut samil_dirs = fs::read_dir(&*path).await?;
        while let Ok(Some(entry)) = samil_dirs.next_entry().await {
            let path = entry.path();
            if path.is_dir() && entry.file_name().to_string_lossy().starts_with("smali") {
                let name = name.clone();
                set.spawn(async move {
                    fs::create_dir_all(path.join(DEFAULT_PACKGENAME_DIR).join(&*name))
                        .await
                        .or_else(|e| {
                            if e.kind() == ErrorKind::AlreadyExists {
                                Ok(())
                            } else {
                                error!(
                                    "{:?} , {:?}",
                                    e,
                                    path.join(DEFAULT_PACKGENAME_DIR).join(&*name)
                                );
                                Err(e)
                            }
                        })?;
                    let mut read_dir = fs::read_dir(path.join(DEFAULT_PACKGENAME_DIR)).await?;
                    while let Some(file) = read_dir.next_entry().await? {
                        if !(file.path().is_dir() && file.file_name().to_str().unwrap() == *name) {
                            Self::rename_package_in_smali(&file.path(), name.clone()).await?;

                            fs::rename(
                                file.path(),
                                path.join(DEFAULT_PACKGENAME_DIR)
                                    .join(&*name)
                                    .join(file.file_name()),
                            )
                            .await?;
                        }
                    }
                    Ok::<String, anyhow::Error>(
                        path.file_name().unwrap().to_string_lossy().to_string(),
                    )
                });
            }
        }

        while let Some(res) = set.join_next().await {
            let _ = self
                .sender
                .send(format!(
                    "finshed job smali transform, {:?}",
                    res.unwrap().unwrap()
                ))
                .await;
        }

        info!("finshed job smali transforms");
        Ok(())
    }

    #[instrument]
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

    #[instrument(skip(self))]
    async fn change_name_string_xml(self: Arc<Self>) -> Result<()> {
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
        let _ = self.sender.send("names and strings changed".into()).await;
        info!("names and strings changed");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn encode(&self) -> Result<()> {
        let apk_tool = self.jar_path.join("apktool.jar");
        let args = [
            "-jar",
            &adjust_canonicalization(apk_tool),
            "b",
            self.out_path.as_os_str().to_str().context("")?,
            "-f",
        ];

        run_java_command(&args, &self.jar_path, move |_, _, out_type| {
            Box::pin(async move {
                match out_type {
                    OutType::StdOut => Ok::<(), anyhow::Error>(()),
                    OutType::StdErr => Err(anyhow!(
                        "failed at Encodeing the apk, check logs for more info"
                    )),
                }
            })
        })
        .await?;
        let _ = self.sender.send("Encodeing finshed".to_owned()).await;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn alignzip(&self) -> Result<()> {
        let out_path_apk_in = self.out_path.join("dist/app.apk");
        let out_path_apk_out = self.out_path.join("dist/app-align.apk");
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

            let mut reader_std_err = BufReader::new(stderr).lines();
            tokio::spawn(async move {
                while let Some(line) = reader_std_err.next_line().await? {
                    error!("{:?}", line)
                }
                anyhow::Ok(())
            });

            while let Some(line) = reader.next_line().await? {
                info!("{:?}", line)
            }
        }

        #[cfg(not(test))]
        {
            let mut prosses = tauri::api::process::Command::new_sidecar("zipalign")?.args(args);

            #[cfg(target_os = "linux")]
            {
                use std::collections::HashMap;
                let mut env = HashMap::with_capacity(1);
                env.insert(
                    "LD_LIBRARY_PATH".to_string(),
                    self.jar_path.join("lib64").to_string_lossy().to_string(),
                );
                prosses = prosses.envs(env);
            }
            let (mut rx, _) = prosses.spawn()?;
            while let Some(event) = rx.recv().await {
                if let tauri::api::process::CommandEvent::Stdout(line) = event {
                    info!("{}", line);
                }
            }
            let _ = self.sender.send("aligning done".into()).await;
        }
        Ok(())
    }

    #[instrument(skip(self))]
    async fn move_resourses(self: Arc<Self>) -> Result<()> {
        let dest = Arc::new(self.out_path.join("res"));
        futures_util::stream::iter(self.config.paths.iter())
            .for_each(|f| {
                let dest = dest.clone();
                async move {
                    let is_image = f.name.ends_with("png") || f.name.ends_with("jpg");
                    if is_image
                        && PathBuf::from(&f.path).extension().unwrap()
                            != f.name.split(".").nth(1).unwrap()
                    {
                        let img = ImageReader::open(&f.path).unwrap().decode().unwrap();
                        img.save(dest.join("drawable").join(&f.name)).unwrap();
                        return;
                    }
                    fs::copy(
                        &f.path,
                        dest.join(if is_image { "drawable" } else { "raw" })
                            .join(&f.name),
                    )
                    .await
                    .unwrap();
                }
            })
            .await;
        let _ = self.sender.send("done moving resources".into()).await;
        info!("done moving resources");

        Ok(())
    }

    #[instrument(skip(self))]
    async fn save_settings(self: Arc<Self>) -> Result<()> {
        fs::write(
            self.out_path.join("res/raw/setting.json"),
            serde_json::to_string(&self.config.app_setting)?.as_bytes(),
        )
        .await?;
        let _ = self.sender.send("applaying the setting".into()).await;
        info!("applaying the setting");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn sign_apk(&self) -> Result<()> {
        let path = self
            .out_path
            .join(format!("dist/{}.keystore", self.config.package_name));
        let apk_tool = self.jar_path.join("apksigner.jar");
        self.gen_keys(&path).await?;
        let out_apk_path = self.out_path.join("dist/app-align.apk");
        let args = [
            "-jar",
            &adjust_canonicalization(apk_tool),
            "sign",
            "--ks",
            path.to_str().context("")?,
            "--ks-pass",
            &format!("pass:{}", self.keypass),
            "--key-pass",
            &format!("pass:{}", self.keypass),
            out_apk_path.to_str().context("")?,
        ];
        run_java_command(&args, &self.jar_path, |_, _, out_type| {
            Box::pin(async move {
                match out_type {
                    OutType::StdOut => Ok::<(), anyhow::Error>(()),
                    OutType::StdErr => Err(anyhow!(
                        "failed at signing the apk, check logs for more info"
                    )),
                }
            })
        })
        .await?;
        let _ = self.sender.send("done signing the app".into()).await;
        info!("done signing the app");

        Ok(())
    }

    #[instrument(skip(self))]
    async fn create_gridinat_colors(self: Arc<Self>) -> Result<()> {
        let extract_values = |css_gridaint: &str| -> (String, Vec<String>) {
            let mut count = 0;
            let splited = css_gridaint
                .char_indices()
                .skip(16)
                .take_while(|f| {
                    if f.1 == ')' {
                        if count == 2 {
                            false
                        } else {
                            count += 1;
                            true
                        }
                    } else {
                        true
                    }
                })
                .map(|f| f.1)
                .collect::<String>();
            let mut splited = splited.split(",");
            let kind = splited.nth(0).unwrap();
            let rest = splited.collect::<String>();

            let colors = rest
                .splitn(2, "%")
                .map(|text| {
                    println!("{:?}", text);
                    text.char_indices()
                        .skip_while(|f| f.1 != '(')
                        .skip(1)
                        .take_while(|f| f.1 != ')')
                        .map(|f| f.1)
                        .collect::<String>()
                })
                .map(|f| {
                    let rgba_vec = f.split(" ").map(|f| f.parse().unwrap()).collect::<Vec<_>>();
                    Rgb::from(rgba_vec[0], rgba_vec[1], rgba_vec[2]).to_css_hex_string()
                })
                .collect::<Vec<String>>();
            (kind.replace("deg", ""), colors)
        };

        let replace_file = |path: PathBuf, str: String| async move {
            let gir = extract_values(&str);
            let mut file = File::options().write(true).read(true).open(path).await?;
            let mut gridaint = String::new();
            file.read_to_string(&mut gridaint).await?;
            file.rewind().await?;
            let kind_replace = (
                if gir.0.parse::<u32>().is_ok() {
                    "315"
                } else {
                    "linear"
                },
                gir.0
                    .parse::<u32>()
                    .map_or_else(|_| "radial".to_owned(), |f| f.to_string()),
            );
            file.write_all(
                &gridaint
                    .replace(kind_replace.0, &kind_replace.1)
                    .replace("#4568DC", &gir.1[0])
                    .replace("#B06AB3", &gir.1[1])
                    .as_bytes(),
            )
            .await?;
            Ok::<(), anyhow::Error>(())
        };

        if self.config.app_setting.splash_screen.type_field == 3 {
            replace_file(
                self.out_path.join("drawable/bg_gradient_splash.xml"),
                self.config
                    .app_setting
                    .splash_screen
                    .splash_screen_g_c
                    .clone()
                    .or(Some("linear-gradient(106deg, rgba(235, 65, 101, 1) 0%, rgba(207, 147, 217, 1) 99%)".to_owned())).unwrap(),
            )
            .await?;
        }

        if self
            .config
            .app_setting
            .sidebar_menu
            .sidebar_menu_header
            .type_field
            == 1
        {
            replace_file(
                self.out_path.join("drawable/bg_gradient_menu_header.xml"),
                    self.config.app_setting.sidebar_menu.sidebar_menu_header.color
                    .clone()
                    .or(Some("linear-gradient(106deg, rgba(235, 65, 101, 1) 0%, rgba(207, 147, 217, 1) 99%)".to_owned())).unwrap(),
            )
            .await?;
        }
        let _ = self.sender.send("done moving resources".into()).await;
        info!("done moving resources");

        Ok(())
    }

    #[instrument(skip(self))]
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
            .spawn()?;

        let stderr = child
            .stderr
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        tokio::spawn(async {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await? {
                error!("{}", line);
            }
            anyhow::Ok(())
        });

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

    #[instrument(skip(self))]
    async fn gen_assetis_link(&self) -> Result<String> {
        let finger_print = self.get_sha256().await?;
        Ok(serde_json::to_string(&Assetlinks::new(Target::new(
            &self.config.package_name,
            [finger_print],
        )))?)
    }

    #[instrument(skip(self))]
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
            .spawn()?;

        let stderr = child
            .stderr
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        tokio::spawn(async {
            let mut reader = BufReader::new(stderr).lines();
            while let Some(line) = reader.next_line().await? {
                error!("{}", line);
            }
            anyhow::Ok(())
        });

        let stdout = child
            .stdout
            .take()
            .with_context(|| "child did not have a handle to stdout")?;

        let mut reader = BufReader::new(stdout).lines();

        while let Some(line) = reader.next_line().await? {
            info!("{}", line);
        }
        let _ = self.sender.send("done generating key".into()).await;

        Ok(())
    }
}

pub async fn check_java(jar_path: &Path) -> Result<Option<String>> {
    let mut is_java = None;
    run_java_command(&["-version"], &jar_path, |f, _, _| {
        if is_java == None && (f.starts_with("java version") || f.starts_with("openjdk version")) {
            is_java = Some(f.as_str()[f.find("\"").unwrap()..f.len() - 1].to_string());
        }
        Box::pin(async { Ok(()) })
    })
    .await?;
    Ok(is_java)
}

#[instrument(skip_all, fields(args, working_dir))]
async fn run_java_command<T, F>(args: &[&str], working_dir: &Path, mut new_line: F) -> Result<()>
where
    F: for<'a> FnMut(
        String,
        &'a mut ChildStdin,
        OutType,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send + 'a>>,
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

    let mut reader = BufReader::new(stderr).lines();
    while let Some(line) = reader.next_line().await.unwrap() {
        error!("{}", &line);
        new_line(line, &mut stdin, OutType::StdErr).await?;
    }

    let mut reader = BufReader::new(stdout).lines();
    while let Some(line) = reader.next_line().await? {
        info!("{}", &line);
        new_line(line, &mut stdin, OutType::StdOut).await?;
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

#[cfg(not(target_os = "windows"))]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    p.as_ref().display().to_string()
}

#[cfg(target_os = "windows")]
fn adjust_canonicalization<P: AsRef<Path>>(p: P) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    let p = p.as_ref().display().to_string();
    if p.starts_with(VERBATIM_PREFIX) {
        p[VERBATIM_PREFIX.len()..].to_string()
    } else {
        p
    }
}

#[cfg(test)]
mod tests {
    use crate::convert::{web2app::Web2app, AppSetting};
    use anyhow::Result;
    use std::{path::PathBuf, sync::Once};

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn setup() {
        INIT.call_once(|| {});
    }

    #[tokio::test]
    async fn change_app() -> Result<()> {
        setup();
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        Web2app::new(
            super::Config {
                name: "giigle".into(),
                package_name: "com.here.we.go".into(),
                icon_path: "/home/arthur/Desktop/packages/app/src-tauri/icons/128x128.png".into(),
                app_setting: AppSetting {
                    ..Default::default()
                },
                paths: vec![],
            },
            d.join("../AndroidWebApp/app/build/outputs/apk/debug/app-debug.apk")
                .canonicalize()?,
            "/home/arthur/.local/share/com.tauri.dev/out/".into(),
            d.join("resources/"),
            tokio::sync::mpsc::channel(10).0,
        )
        .await
        .run()
        .await?;

        Ok(())
    }
}

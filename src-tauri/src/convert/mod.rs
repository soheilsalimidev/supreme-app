use derive_new::new;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

pub mod web2app;

#[derive(Serialize, Deserialize, new)]
pub struct Assetlinks<'a> {
    #[serde(rename = "relation")]
    #[new(value = "[\"delegate_permission/common.handle_all_urls\"]")]
    relation: [&'a str; 1],

    #[serde(borrow)]
    #[serde(rename = "target")]
    target: Target<'a>,
}

#[derive(Serialize, Deserialize, new)]
pub struct Target<'a> {
    #[serde(rename = "namespace")]
    #[new(value = "\"android_app\"")]
    namespace: &'a str,

    #[serde(rename = "package_name")]
    package_name: &'a str,

    #[serde(rename = "sha256_cert_fingerprints")]
    sha256_cert_fingerprints: [String; 1],
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoogleService {
    #[serde(rename = "project_info")]
    pub project_info: ProjectInfo,
    pub client: Vec<Client>,
    #[serde(rename = "configuration_version")]
    pub configuration_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    #[serde(rename = "project_number")]
    pub project_number: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "storage_bucket")]
    pub storage_bucket: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    #[serde(rename = "client_info")]
    pub client_info: ClientInfo,
    #[serde(rename = "oauth_client")]
    pub oauth_client: Vec<Value>,
    #[serde(rename = "api_key")]
    pub api_key: Vec<ApiKey>,
    pub services: Services,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    #[serde(rename = "mobilesdk_app_id")]
    pub mobilesdk_app_id: String,
    #[serde(rename = "android_client_info")]
    pub android_client_info: AndroidClientInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AndroidClientInfo {
    #[serde(rename = "package_name")]
    pub package_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    #[serde(rename = "current_key")]
    pub current_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    #[serde(rename = "appinvite_service")]
    pub appinvite_service: AppinviteService,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppinviteService {
    #[serde(rename = "other_platform_oauth_client")]
    pub other_platform_oauth_client: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    #[serde(rename = "site_url")]
    pub site_url: String,
    #[serde(rename = "splash_screen")]
    pub splash_screen: i64,
    #[serde(rename = "splash_screen_g_c")]
    pub splash_screen_g_c: i64,
    #[serde(rename = "cache_mode")]
    pub cache_mode: i64,
    #[serde(rename = "no_internet_layout")]
    pub no_internet_layout: i64,
    pub toolbar: i64,
    #[serde(rename = "toolbar_custom_icon")]
    pub toolbar_custom_icon: Option<Pair>,
    #[serde(rename = "sidebar_menu")]
    pub sidebar_menu: i64,
    #[serde(rename = "sidebar_menu_header_mode")]
    pub sidebar_menu_header_mode: i64,
    #[serde(rename = "sidebar_menu_header_color")]
    pub sidebar_menu_header_color: i64,
    #[serde(rename = "sidebar_menu_footer_mode")]
    pub sidebar_menu_footer_mode: i64,
    #[serde(rename = "swipe_refresh")]
    pub swipe_refresh: i64,
    pub admob: i64,
    #[serde(rename = "admob_banner")]
    pub admob_banner: i64,
    #[serde(rename = "floating_action_button_menu")]
    pub floating_action_button_menu: i64,
    pub google_service: Option<GoogleService>,
    #[serde(rename = "item_menu")]
    pub item_menu: Vec<ItemMenu>,
    #[serde(rename = "item_fab")]
    pub item_fab: Vec<ItemMenu>,
    #[serde(rename = "intro_pages")]
    pub intro_pages: Vec<IntroPage>,
    #[serde(rename = "introPage")]
    pub intro_page: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemMenu {
    #[serde(rename = "Pair")]
    pub pair: Option<Pair>,
    #[serde(rename = "Kind")]
    pub kind: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub first: String,
    pub second: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntroPage {
    pub title: String,
    pub description: String,
    pub background: i64,
    #[serde(rename = "image_name")]
    pub image_name: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            site_url: "https://docs.rs/passwords/latest/passwords/".to_owned(),
            splash_screen: 1,
            splash_screen_g_c: 1,
            cache_mode: 1,
            no_internet_layout: 1,
            toolbar: 1,
            toolbar_custom_icon: None,
            sidebar_menu: 1,
            sidebar_menu_header_mode: 1,
            sidebar_menu_header_color: 1,
            sidebar_menu_footer_mode: 1,
            swipe_refresh: 1,
            admob: 1,
            admob_banner: 1,
            floating_action_button_menu: 1,
            google_service: None,
            intro_pages: vec![IntroPage {
                title: "sdl".into(),
                description: "asd".into(),
                background: 1,
                image_name: "sad".into(),
            }],
            item_menu: vec![ItemMenu {
                pair: Some(Pair {
                    first: "das".into(),
                    second: "asd".into(),
                }),
                kind: None,
            }],
            item_fab: vec![ItemMenu {
                pair: Some(Pair {
                    first: "das".into(),
                    second: "asd".into(),
                }),
                kind: None,
            }],
            intro_page: false,
        }
    }
}

struct Config {
    name: String,
    package_name: String,
    icon_path: PathBuf,
    app_setting: Setting,
    images_path: Vec<PathBuf>,
}

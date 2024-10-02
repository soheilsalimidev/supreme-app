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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colors {
    pub metarial: bool,
    pub primary: String,
    pub light: Option<ColorSchema>,
    pub dark: Option<ColorSchema>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSchema {
    pub primary: String,
    pub on_primary: String,
    pub primary_container: String,
    pub on_primary_container: String,
    pub secondary: String,
    pub on_secondary: String,
    pub secondary_container: String,
    pub on_secondary_container: String,
    pub tertiary: String,
    pub on_tertiary: String,
    pub tertiary_container: String,
    pub on_tertiary_container: String,
    pub error: String,
    pub on_error: String,
    pub error_container: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSetting {
    colors: Colors,
    #[serde(rename = "site_url")]
    pub site_url: String,
    #[serde(rename = "aboutUs")]
    about_us: AboutUS,
    #[serde(rename = "splash_screen")]
    pub splash_screen: SplashScreen,
    #[serde(rename = "cache_mode")]
    pub cache_mode: i64,
    #[serde(rename = "no_internet_layout")]
    pub no_internet_layout: NoInternetLayout,
    pub toolbar: Toolbar,
    #[serde(rename = "toolbar_custom_icon")]
    pub toolbar_custom_icon: ToolbarCustomIcon,
    #[serde(rename = "swipe_refresh")]
    pub swipe_refresh: bool,
    #[serde(rename = "sidebar_menu")]
    pub sidebar_menu: SidebarMenu,
    pub admob: i64,
    #[serde(rename = "admob_banner")]
    pub admob_banner: i64,
    #[serde(rename = "floating_action_button")]
    pub floating_action_button: FloatingActionButton,
    pub intro_page: IntroPage,
    google_service: Option<GoogleService>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutUS {
    #[serde(rename = "enable")]
    pub enable: bool,
    #[serde(rename = "text")]
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplashScreen {
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "splash_screen_g_c")]
    pub splash_screen_g_c: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoInternetLayout {
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "lottieFile")]
    lottie_file: Option<String>,
    image: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Toolbar {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolbarCustomIcon {
    pub enable: bool,
    pub first: Option<String>,
    pub second: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarMenu {
    pub enable: bool,
    #[serde(rename = "sidebar_menu_header")]
    pub sidebar_menu_header: SidebarMenuHeader,
    #[serde(rename = "sidebar_menu_footer")]
    pub sidebar_menu_footer: SidebarMenuFooter,
    #[serde(rename = "item_menu")]
    pub item_menu: Vec<ItemMenu>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarMenuHeader {
    #[serde(rename = "type")]
    pub type_field: i64,
    color: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidebarMenuFooter {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemMenu {
    #[serde(rename = "Kind")]
    pub kind: Option<i64>,
    #[serde(rename = "Pair")]
    pub pair: Option<Pair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub first: String,
    pub second: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatingActionButton {
    pub enable: bool,
    #[serde(rename = "item_fab")]
    pub item_fab: Vec<ItemMenu>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntroPage {
    pub enable: bool,
    pub pages: Vec<Page>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    title: String,
    description: String,
    background: String,
    image_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagesPath {
    pub key: String,
}

impl Default for AppSetting {
    fn default() -> Self {
        Self {
            colors: Colors {
                metarial: false,
                primary: "".to_owned(),
                light: None,
                dark: None,
            },
            site_url: "https://docs.rs/passwords/latest/passwords/".to_owned(),
            splash_screen: SplashScreen {
                type_field: 1,
                splash_screen_g_c: None,
            },
            cache_mode: 1,
            no_internet_layout: NoInternetLayout {
                type_field: 0,
                lottie_file: None,
                image: None,
            },
            toolbar: Toolbar {
                type_field: 1,
                text: None,
            },
            toolbar_custom_icon: ToolbarCustomIcon {
                enable: false,
                first: None,
                second: None,
            },
            sidebar_menu: SidebarMenu {
                enable: false,
                sidebar_menu_header: SidebarMenuHeader {
                    type_field: 0,
                    color: None,
                },
                sidebar_menu_footer: SidebarMenuFooter {
                    type_field: 0,
                    text: None,
                },
                item_menu: vec![
                    ItemMenu {
                        kind: Some(1),
                        pair: None,
                    },
                    ItemMenu {
                        kind: None,
                        pair: Some(Pair {
                            first: "asd".into(),
                            second: "sad".into(),
                        }),
                    },
                ],
            },
            swipe_refresh: true,
            admob: 1,
            admob_banner: 1,
            google_service: None,
            intro_page: IntroPage {
                enable: false,
                pages: vec![],
            },
            floating_action_button: FloatingActionButton {
                enable: false,
                item_fab: vec![
                    ItemMenu {
                        kind: Some(1),
                        pair: None,
                    },
                    ItemMenu {
                        kind: None,
                        pair: Some(Pair {
                            first: "asd".into(),
                            second: "sad".into(),
                        }),
                    },
                ],
            },
            about_us: AboutUS {
                enable: false,
                text: None,
            },
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    package_name: String,
    icon_path: PathBuf,
    app_setting: AppSetting,
    pub paths: Vec<Paths>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Paths {
    pub(super) name: String,
    pub(super) path: String,
}

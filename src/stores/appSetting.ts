import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useAppSettingStore = defineStore("appSetting", () => {
  const appInfo = ref<AppInfo>({
    name: "",
    package_name: "",
    icon_path: "",
    app_setting: {
      site_url: "",
      splash_screen: {
        type:0,
      },
      splash_screen_g_c: "",
      cache_mode: 0,
      no_internet_layout: 0,
      toolbar: 0,
      sidebar_menu: 0,
      sidebar_menu_header_mode: 0,
      sidebar_menu_header_color: 0,
      sidebar_menu_footer_mode: 0,
      swipe_refresh: 0,
      admob: 0,
      introPage: false,
      admob_banner: 0,
      floating_action_button_menu: 0,
      googleService: undefined,
      item_menu: [],
      item_fab: [],
      intro_pages: [],
    },
    images_path: [
      {
        key: "splash",
        file: undefined,
      },
    ],
  });

  const get_logo = computed(() => {
    if (appInfo.value.icon_path) {
      return URL.createObjectURL((appInfo.value.icon_path as File[])[0]);
    }
    return "";
  });

  return { appInfo, get_logo };
});

export interface AppInfo {
  name: string;
  package_name: string;
  icon_path: string | File[];
  app_setting: Setting;
  images_path: { key: string; file: File }[];
}

export interface Setting {
  site_url: string;
  splash_screen: {
    type: number;
    splash_screen_g_c?: string;
    image_path?: string;
  };
  cache_mode: number;
  no_internet_layout: number;
  toolbar: number;
  toolbar_custom_icon?: ToolbarCustomIcon;
  sidebar_menu: number;
  sidebar_menu_header_mode: number;
  sidebar_menu_header_color: number;
  sidebar_menu_footer_mode: number;
  swipe_refresh: number;
  admob: number;
  introPage: boolean;
  admob_banner: number;
  floating_action_button_menu: number;
  googleService: any;
  item_menu: ItemMenu[];
  item_fab: ItemMenu[];
  intro_pages: IntroPage[];
}

export interface ToolbarCustomIcon {
  first: string;
  second: string;
}

export interface ItemMenu {
  Pair?: Pair;
  Kind?: number;
}

export interface Pair {
  first: string;
  second: string;
}

export interface IntroPage {
  title: string;
  description: string;
  background: number;
  image_name: string;
}

import { defineStore } from "pinia";
import { ref, watch } from "vue";

export const useAppSettingStore = defineStore("appSetting", () => {
  const appInfo = ref<AppInfo>({
    name: "",
    package_name: "",
    icon_path: "",
    app_setting: {
      site_url: "",
      splash_screen: {
        type: 0,
      },
      cache_mode: 0,
      no_internet_layout: {
        type: 1,
      },
      toolbar: {
        type: 0,
      },
      toolbar_custom_icon: {
        enable: false,
      },
      swipe_refresh: false,
      sidebar_menu: {
        enable: false,
        sidebar_menu_header: {
          type: 0,
        },
        sidebar_menu_footer: {
          type: 0,
        },
        item_menu: [],
      },
      admob: 0,
      admob_banner: 0,
      floating_action_button: {
        enable: false,
        item_fab: [],
      },
      googleService: undefined,
      introPage: {
        enable: false,
        pages: [],
      },
      aboutUs: {
        enable: false,
        text: "",
      },
    },
    paths: [],
  });

  watch(appInfo, () => console.log(appInfo.value));

  return { appInfo };
});

export interface AppInfo {
  name: string;
  package_name: string;
  icon_path: string;
  app_setting: Setting;
  paths: { name: string; path: string }[];
}

export interface Setting {
  site_url: string;
  aboutUs: {
    enable: boolean;
    text: string;
  };
  splash_screen: {
    type: number;
    splash_screen_g_c?: string;
    image_path?: string;
  };
  cache_mode: number;
  no_internet_layout: {
    type: number;
    lottieFile?: string;
    image?: string;
  };
  toolbar: {
    type: number;
    text?: string;
  };
  toolbar_custom_icon: ToolbarCustomIcon;
  sidebar_menu: {
    enable: boolean;
    sidebar_menu_header: {
      type: 0 | 1;
      color?: string;
    };
    sidebar_menu_footer: {
      type: 0 | 1;
      text?: string;
    };
    item_menu: ItemMenu[];
  };
  swipe_refresh: boolean;
  admob: number;
  admob_banner: number;
  googleService: any;
  floating_action_button: {
    enable: boolean;
    item_fab: ItemMenu[];
  };
  introPage: {
    enable: boolean;
    pages: IntroPage[];
  };
}

export interface ToolbarCustomIcon {
  enable: boolean;
  first?: string;
  second?: string;
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

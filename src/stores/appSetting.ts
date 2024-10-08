import { defineStore } from "pinia";
import { ref } from "vue";

export const useAppSettingStore = defineStore("appSetting", () => {
  const savePath = ref();
  const openedPageIndexIntro = ref(0);
  const selectedWebPageSetting = ref([
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);

  // const test: AppInfo = {
  //   name: "asdf adsfasd",
  //   package_name: "asdf.adsfasd",
  //   icon_path: "C:\\Users\\sohei\\OneDrive\\Desktop\\supreme-app\\src\\assets\\no_internet.png",
  //   app_setting: {
  //     googleService: undefined,
  //     site_url:
  //       "https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html",
  //     splash_screen: {
  //       type: 0,
  //     },
  //     cache_mode: 0,
  //     no_internet_layout: {
  //       type: 1,
  //     },
  //     toolbar: {
  //       type: 0,
  //     },
  //     toolbar_custom_icon: {
  //       enable: false,
  //     },
  //     swipe_refresh: false,
  //     sidebar_menu: {
  //       enable: true,
  //       sidebar_menu_header: {
  //         type: 0,
  //       },
  //       sidebar_menu_footer: {
  //         type: 1,
  //         text: "sadfsda",
  //       },
  //       item_menu: [
  //         {
  //           Kind: 2,
  //         },
  //         {
  //           Pair: {
  //             first: "asdf",
  //             second: "sadf",
  //           },
  //         },
  //       ],
  //     },
  //     admob: 0,
  //     admob_banner: 0,
  //     floating_action_button: {
  //       enable: false,
  //       item_fab: [],
  //     },
  //     introPage: {
  //       enable: false,
  //       pages: [],
  //     },
  //     aboutUs: {
  //       enable: false,
  //       text: "",
  //     },
  //   },
  //   paths: [
  //     {
  //       path: "C:\\Users\\sohei\\OneDrive\\Desktop\\supreme-app\\src\\assets\\no_internet.png",
  //       name: "logo.png",
  //     },
  //   ],
  // };
  const appInfo = ref<AppInfo>(
    // test,
    {
      name: "",
      package_name: "",
      icon_path: "",
      colors: {
        metarial: true,
        primary: "",
        light: {},
        dark: {},
      },
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
    },
  );

  let lastUpdateFile = JSON.stringify(appInfo.value);

  return {
    appInfo,
    savePath,
    selectedWebPageSetting,
    openedPageIndexIntro,
    lastUpdateFile,
  };
});

export interface AppInfo {
  name: string;
  package_name: string;
  icon_path: string;
  colors: {
    dark: Partial<ISchema>;
    light: Partial<ISchema>;
    primary: string;
    metarial: boolean;
  };
  app_setting: Setting;
  paths: { name: string; path: string }[];
}

interface ISchema {
  primary: string;
  onPrimary: string;
  primaryContainer: string;
  onPrimaryContainer: string;
  secondary: string;
  onSecondary: string;
  secondaryContainer: string;
  onSecondaryContainer: string;
  tertiary: string;
  onTertiary: string;
  tertiaryContainer: string;
  onTertiaryContainer: string;
  error: string;
  onError: string;
  errorContainer: string;
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
  loading?: string;
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
  background: string;
  imageName: string;
}

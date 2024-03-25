<script setup lang="ts">
import LineMdHomeMd from "~icons/line-md/home-md";
import LineMdPhone from "~icons/line-md/phone";
import LineMdStarAltFilled from "~icons/line-md/star-alt-filled";
import LineMdExternalLink from "~icons/line-md/external-link";
import LineMdExternalLinkRounded from "~icons/line-md/external-link-rounded";
import { useAppSettingStore } from "@/stores/appSetting";
import noInternet from "@/assets/no_internet.png";
import HeroiconsPlus20Solid from "~icons/heroicons/plus-20-solid";
import { storeToRefs } from "pinia";
import HeroiconsBars3 from "~icons/heroicons/bars-3";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import LineMdLoadingLoop from "~icons/line-md/loading-loop";
import noInternetJson from "@/assets/no_internet.json";
import loadingJson from "@/assets/loading.json";
import { DotLottieVue } from "@lottiefiles/dotlottie-vue";
import { invoke } from "@tauri-apps/api";
import { computedAsync } from "@vueuse/core";
import { computed } from "vue";
const { appInfo, selectedWebPageSetting } = storeToRefs(useAppSettingStore());

const getJsonDateForLoaing = computedAsync(async () => {
  return appInfo.value.app_setting.loading
    ? await invoke<string>("get_file", {
        path: appInfo.value.app_setting.loading,
      })
    : JSON.stringify(loadingJson);
}, JSON.stringify(loadingJson));

const getJsonDateForNoInter = computedAsync(async () => {
  return appInfo.value.app_setting.no_internet_layout.lottieFile
    ? await invoke<string>("get_file", {
        path: appInfo.value.app_setting.no_internet_layout.lottieFile,
      })
    : JSON.stringify(noInternetJson);
}, JSON.stringify(noInternetJson));

const getMenuItem = computed(() => {
  return selectedWebPageSetting.value[5]
    ? appInfo.value.app_setting.sidebar_menu.item_menu
    : [];
});

const getFabItem = computed(() => {
  return selectedWebPageSetting.value[6]
    ? appInfo.value.app_setting.floating_action_button.item_fab
    : [];
});

const defaultItems = [
  {
    name: "home_page",
    icon: LineMdHomeMd,
  },
  {
    name: "about_us",
    icon: LineMdPhone,
  },
  {
    name: "rate_us",
    icon: LineMdStarAltFilled,
  },
  {
    name: "share_app",
    icon: LineMdExternalLink,
  },
  {
    name: "exit",
    icon: LineMdExternalLinkRounded,
  },
  {
    name: "custom",
  },
];
</script>

<template>
  <div class="relative h-full w-full z-10 overflow-hidden">
    <div
      v-if="
        appInfo.app_setting.sidebar_menu.enable && selectedWebPageSetting[5]
      "
      class="absolute top-[1.7rem] bottom-0 start-0 h-full z-50"
    >
      <aside class="mdc-drawer !w-56">
        <div
          class="mdc-drawer__header !p-0 !m-0 h-24"
          v-if="appInfo.app_setting.sidebar_menu.sidebar_menu_header.type === 1"
          :style="{
            'background-image':
              appInfo.app_setting.sidebar_menu.sidebar_menu_header.color,
          }"
        >
          <h3 class="mdc-drawer__title !text-white">
            {{ appInfo.name }}
          </h3>
        </div>
        <div class="mdc-drawer__content">
          <nav class="mdc-deprecated-list flex flex-col h-full">
            <TransitionGroup name="list">
              <a
                v-for="(item, index) in getMenuItem"
                :key="index"
                class="mdc-deprecated-list-item"
                :class="index === 0 && 'mdc-deprecated-list-item--activated'"
              >
                <span class="mdc-deprecated-list-item__ripple"></span>
                <component
                  :is="defaultItems[item.Kind - 1].icon"
                  v-if="item.Kind"
                  class="material-icons mdc-deprecated-list-item__graphic"
                >
                  <LineMdExternalLinkRounded class="text-white text-xl">
                  </LineMdExternalLinkRounded
                ></component>

                <span class="mdc-deprecated-list-item__text">{{
                  item.Kind
                    ? defaultItems[item.Kind - 1].name
                    : item.Pair?.first
                }}</span>
              </a>

              <a
                v-if="
                  appInfo.app_setting.sidebar_menu.sidebar_menu_footer.type ===
                  1
                "
                class="mdc-deprecated-list-item !mt-auto !mb-4"
              >
                <span class="mdc-deprecated-list-item__ripple"></span>

                <span class="mdc-deprecated-list-item__text"
                  >{{
                    appInfo.app_setting.sidebar_menu.sidebar_menu_footer.text
                  }}
                </span>
              </a>
            </TransitionGroup>
          </nav>
        </div>
      </aside>
    </div>

    <div class="flex flex-col bg-white h-full w-full rounded-md z-10">
      <div class="bg-indigo-700 h-7 text-white px-2 rounded-t-md">
        {{ $t("frames.web_page_frame.11_11") }}
      </div>
      <div
        v-if="appInfo.app_setting.toolbar.type === 1"
        class="bg-indigo-500 h-[54px] flex items-center px-2"
      >
        <HeroiconsBars3 class="h-8 w-8 text-white">
          <p class="text-white ms-2 text-lg font-bold">
            {{ appInfo.app_setting.toolbar.text }}
          </p>
          <img
            v-if="
              appInfo.app_setting.toolbar_custom_icon.enable &&
              appInfo.app_setting.toolbar_custom_icon.first
            "
            class="w-10 ms-auto"
            :src="convertFileSrc(appInfo.app_setting.toolbar_custom_icon.first)"
          />
        </HeroiconsBars3>
      </div>
      <div
        v-if="
          selectedWebPageSetting.slice(1).every((item) => !item) &&
          appInfo.app_setting.swipe_refresh
        "
        class="relative shadow-gray-200 h-8 w-8 rounded-full shadow-md bg-stone-200 top-5 left-1/2 -translate-x-1/2 flex justify-center items-center"
      >
        <LineMdLoadingLoop> </LineMdLoadingLoop>
      </div>
      <div
        class="flex h-full w-full justify-center items-center flex-col relative"
      >
        <div
          v-if="selectedWebPageSetting[7]"
          class="flex flex-col justify-center items-center"
        >
          <DotLottieVue
            :key="getJsonDateForLoaing"
            class="h-64"
            :data="getJsonDateForLoaing"
            :autoplay="true"
            :loop="true"
            :speed="1"
          >
          </DotLottieVue>
        </div>

        <div
          v-else-if="selectedWebPageSetting[2]"
          class="flex flex-col justify-center items-center"
        >
          <img
            v-if="appInfo.app_setting.no_internet_layout.type === 1"
            class="p-8"
            :src="
              appInfo.app_setting.no_internet_layout.image
                ? convertFileSrc(appInfo.app_setting.no_internet_layout.image)
                : noInternet
            "
          />
          <DotLottieVue
            v-else
            :key="getJsonDateForNoInter"
            class="h-64"
            :data="getJsonDateForNoInter"
            :autoplay="true"
            :loop="true"
            :speed="1"
          >
            <h2 class="font-bold text-indigo-500 text-2xl">
              {{ $t("frames.web_page_frame.w_h_o_o_o_p_s") }}
            </h2>
          </DotLottieVue>
        </div>
        <p v-else>
          {{ $t("frames.web_page_frame.your_site_content") }}
        </p>
      </div>

      <div
        v-if="appInfo.app_setting.floating_action_button.enable"
        class="relative bottom-3 start-2 flex flex-col justify-center items-center w-14"
      >
        <TransitionGroup tag="div" name="list">
          <button
            v-for="(item, index) in getFabItem"
            :key="index"
            class="p-0 m-2 w-10 h-10 bg-indigo-500 rounded-full hover:bg-indigo-700 active:shadow-lg mouse shadow transition ease-in duration-200 focus:outline-none flex justify-center items-center"
          >
            <component
              :is="defaultItems[item.Kind - 1].icon"
              v-if="item.Kind"
              class="text-white text-xl"
            >
              <LineMdExternalLinkRounded class="text-white text-xl">
              </LineMdExternalLinkRounded
            ></component>
          </button>
        </TransitionGroup>

        <button
          class="p-0 w-14 h-14 bg-indigo-600 rounded-full hover:bg-indigo-700 active:shadow-lg mouse shadow transition ease-in duration-200 focus:outline-none flex justify-center items-center"
        >
          <HeroiconsPlus20Solid class="text-white text-2xl">
          </HeroiconsPlus20Solid>
        </button>
      </div>
      <div class="bg-black/20 w-full h-full absolute top-0 start-0 transition"  v-if="appInfo.app_setting.sidebar_menu.enable && selectedWebPageSetting[5]"></div>
    </div>
  </div>
</template>

<style lang="scss">
@use "@material/drawer";
@use "@material/list";

@include drawer.core-styles;
@include drawer.dismissible-core-styles;
@include drawer.modal-core-styles;
@include list.deprecated-core-styles;
</style>

<style scoped>
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>

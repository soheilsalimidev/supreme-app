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
// import noInternetJson from "@/assets/no_internet.json";
// import loadingJson from "@/assets/loading.json";
import { DotLottieVue } from "@lottiefiles/dotlottie-vue";
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useDateFormat, useNow } from "@vueuse/core";

const formattedTime = useDateFormat(useNow(), "HH:mm");
const { appInfo, selectedWebPageSetting } = storeToRefs(useAppSettingStore());

const { t } = useI18n({ useScope: "global" });

const getJsonDateForLoaing = computed(() => {
  return appInfo.value.app_setting.loading
    ? convertFileSrc(appInfo.value.app_setting.loading)
    : "/loading.lottie";
});

const getJsonDateForNoInter = computed(() => {
  return appInfo.value.app_setting.no_internet_layout.lottieFile
    ? convertFileSrc(appInfo.value.app_setting.no_internet_layout.lottieFile)
    : "/no_internet.lottie";
});

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

const defaultItemsIcons = [
  LineMdHomeMd,
  LineMdPhone,
  LineMdStarAltFilled,
  LineMdExternalLink,
  LineMdExternalLinkRounded,
];

const defaultItems = ref([
  t("item.home_page"),
  t("item.about_us"),
  t("item.rate_us"),
  t("item.share_app"),
  t("item.exit"),
  t("item.custom"),
]);
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
                  :is="
                    item.Kind
                      ? defaultItemsIcons[item.Kind - 1]
                      : LineMdExternalLinkRounded
                  "
                  class="material-icons mdc-deprecated-list-item__graphic"
                >
                </component>

                <span class="mdc-deprecated-list-item__text">{{
                  item.Kind ? defaultItems[item.Kind - 1] : item.Pair?.first
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
        {{ formattedTime }}
      </div>
      <div
        v-if="appInfo.app_setting.toolbar.type === 1"
        class="bg-indigo-500 h-[54px] flex items-center px-2"
      >
        <HeroiconsBars3 class="h-8 w-8 text-white"> </HeroiconsBars3>
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
      </div>
      <div
        v-if="
          selectedWebPageSetting.slice(1).every((item) => !item) &&
          appInfo.app_setting.swipe_refresh
        "
        class="top-5 w-full flex justify-center items-center left-0 relative"
      >
        <div
          class="shadow-gray-200 h-8 w-8 rounded-full shadow-md bg-stone-200 flex justify-center items-center"
        >
          <LineMdLoadingLoop> </LineMdLoadingLoop>
        </div>
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
            :src="getJsonDateForLoaing"
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
            :src="getJsonDateForNoInter"
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
              v-if="item.Kind"
              :is="defaultItemsIcons[item.Kind - 1]"
              class="text-white text-xl"
            >
            </component>
            <LineMdExternalLinkRounded class="text-white text-xl" v-else>
            </LineMdExternalLinkRounded>
          </button>
        </TransitionGroup>

        <button
          class="p-0 w-14 h-14 bg-indigo-600 rounded-full hover:bg-indigo-700 active:shadow-lg mouse shadow transition ease-in duration-200 focus:outline-none flex justify-center items-center"
        >
          <HeroiconsPlus20Solid class="text-white text-2xl">
          </HeroiconsPlus20Solid>
        </button>
      </div>
      <div
        class="bg-black/20 w-full h-full absolute top-0 start-0 transition"
        v-if="
          appInfo.app_setting.sidebar_menu.enable && selectedWebPageSetting[5]
        "
      ></div>
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

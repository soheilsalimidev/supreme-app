<script setup lang="ts">
import { IntroPage, useAppSettingStore } from "@/stores/appSetting";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
const { t } = useI18n();
const { appInfo, openedPageIndexIntro } = storeToRefs(useAppSettingStore());
const page = computed<IntroPage | undefined>(
  () => appInfo.value.app_setting.introPage.pages[openedPageIndexIntro.value],
);

const getImage = computed(() => {
  try {
    return convertFileSrc(page.value?.imageName ?? "");
  } catch (error) {}
});
</script>

<template>
  <div class="relative h-full w-full">
    <Transition
      v-if="page"
      enter-from-class="translate-x-[100%]"
      leave-active-class="translate-x-[150%]"
      enter-active-class="transition duration-500"
      mode="out-in"
    >
      <div
        :key="openedPageIndexIntro"
        :style="{
          backgroundImage: page?.background,
        }"
        class="h-full w-full overflow-hidden flex items-center justify-center flex-col pt-4"
      >
        <h3 class="text-slate-900 text-2xl font-bold mt-10">
          {{ page?.title }}
        </h3>
        <img :src="getImage" />
        <p class="text-slate-800 text-lg text-center">
          {{ page?.description }}
        </p>
      </div>
    </Transition>
    <div
      v-else
      class="h-full w-full overflow-hidden flex items-center justify-center flex-col pt-4 from-teal-200 to-teal-500 bg-gradient-135"
    >
      <h3 class="font-display text-lg font-bold p-3 text-center text-slate-900">
        {{ t("selectPage") }}
      </h3>
    </div>
    <div
      class="mt-auto flex flex-row-reverse justify-center items-center absolute w-full gap-2 mb-2 bottom-2"
    >
      <span
        class="h-2 w-2 border-white border rounded-full transition duration-300"
        v-for="(_, index) in appInfo.app_setting.introPage.pages"
        :class="index === openedPageIndexIntro && 'bg-white'"
      ></span>
      <span class="start-2 text-white p-3 font-medium absolute cursor-pointer"
        >Next</span
      >
    </div>
  </div>
</template>

<i18n>
  {
  "en":{
  "selectPage":"Please select page to preview"
  }
  }
</i18n>

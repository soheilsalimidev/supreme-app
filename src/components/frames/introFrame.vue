<script setup lang="ts">
import { IntroPage, useAppSettingStore } from "@/stores/appSetting";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const { appInfo, openedPageIndexIntro } = storeToRefs(useAppSettingStore());

const page = computed<IntroPage | undefined>(
  () => appInfo.value.app_setting.introPage.pages[openedPageIndexIntro.value],
);

const getImage = computed(() => {
  try {
    return convertFileSrc(page.value?.image_name ?? "");
  } catch (error) {}
});
</script>

<template>
  <div
    :style="{
      backgroundImage: page?.background,
    }"
    class="h-full w-full overflow-hidden flex items-center justify-center flex-col pt-4"
  >
    <h3 class="text-slate-900 text-2xl font-bold mt-10">
      {{ page?.title }}
    </h3>
    <img :src="getImage" />
    <p class="text-slate-800 text-lg text-center">{{ page?.description }}</p>
    <div class="mt-auto flex flex-row-reverse justify-center items-center relative w-full gap-2 mb-2 ">
      <span
        class="h-2 w-2 border-white border rounded-full"
        v-for="(_, index) in appInfo.app_setting.introPage.pages"
        :class="index === openedPageIndexIntro && 'bg-white'"
      ></span>
      <span class="start-2 text-white p-3 font-medium absolute cursor-pointer">Next</span>
    </div>
  </div>
</template>

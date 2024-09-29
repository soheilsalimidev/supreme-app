<template>
  <div class="w-full h-full flex flex-col items-center justify-center overflow-x-clip">
    <Transition name="fade">
      <div
        v-if="!appInfo.app_setting.introPage.enable"
        class="flex flex-col justify-center items-center"
      >
        <span
          class="text-slate-800 dark:text-slate-200 mb-10 text-center text-base"
        >
          {{ t("introExplain") }}
        </span>
        <div class="flex items-center mb-5 justify-center">
          <label
            for="pageIntro"
            class="me-2 text-lg font-bold text-gray-900 dark:text-gray-300 font-display"
            >{{ t("needIntro") }}</label
          >
          <input
            id="pageIntro"
            type="checkbox"
            value=""
            v-model="appInfo.app_setting.introPage.enable"
            @change="
              appInfo.app_setting.introPage.pages.push({
                title: '',
                description: '',
                background:
                  'linear-gradient(315deg, rgba(59, 44, 231, 1) 0%, rgba(55, 245, 245, 1) 100%)',
                imageName: '',
              })
            "
            class="w-5 h-5 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800"
            required
          />
        </div>
      </div>

      <div v-else class="w-full min-h-full">
        <div class="flex flex-col mt-3 gap-2 relative">
          <TransitionGroup
            enter-from-class="translate-x-[150%] opacity-0"
            enter-active-class="transition duration-300"
            leave-to-class="translate-x-[150%] opacity-0"
            leave-active-class="transition duration-300"
          >
            <IntroPageItem
              v-for="(page, index) in appInfo.app_setting.introPage.pages"
              :page
              :index
            />
          </TransitionGroup>
        </div>

        <button
          v-if="appInfo.app_setting.introPage.enable"
          class="text-gray-900 dark:text-slate-100 rounded-3xl px-2 max-w-16 hover:max-w-40 py-2 text-base font-medium bottom-4 end-4 absolute bg-indigo-500 flex justify-center items-center gap-2 group transition-all duration-500 ease-in-out"
          @click="
            appInfo.app_setting.introPage.pages.push({
              title: '',
              description: '',
              background:
                'linear-gradient(315deg, rgba(59, 44, 231, 1) 0%, rgba(55, 245, 245, 1) 100%)',
              imageName: '',
            })
          "
        >
          <span class="group-hover:block hidden min-w-20"> Add more </span>
          <HeroiconsPlus16Solid class="w-8 h-8" />
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import HeroiconsPlus16Solid from "~icons/heroicons/plus-16-solid";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import IntroPageItem from "../introPageItem.vue";

const { appInfo } = storeToRefs(useAppSettingStore());

const { t } = useI18n();
</script>

<i18n>
  {
  "en":{
    "introExplain":"An intro, short for \"introduction\", is a section at the beginning of an application that briefly explains its purpose, features, and usage to the user. It's a onboarding process that helps new users understand how to navigate and utilize the app. The intro is usually displayed once, after which it is skipped to prevent repetition. Its goal is to provide a smooth and engaging user experience, setting the stage for the user's interaction with the app. It's often visual, interactive, and concise.",
    "needIntro":"I need intro",
  }
}
</i18n>

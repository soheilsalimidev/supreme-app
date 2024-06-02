<template>
  <div class="w-full h-full flex flex-col items-center justify-center">
    <Transition name="fade">
      <div
        v-if="!appInfo.app_setting.introPage.enable"
        class="flex flex-col justify-center items-center"
      >
        <span
          class="text-slate-800 dark:text-slate-200 mb-10 text-center text-base"
        >
          Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit
          enim labore culpa sint ad nisi Lorem pariatur mollit ex esse
          exercitation amet. Nisi anim cupidatat excepteur officia.
          Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate
          voluptate dolor minim nulla est proident. Nostrud officia pariatur ut
          officia. Sit irure elit esse ea nulla sunt ex
        </span>
        <div class="flex items-center mb-5 justify-center">
          <label
            for="pageIntro"
            class="me-2 text-lg font-bold text-gray-900 dark:text-gray-300 font-display"
            >I need Intro</label
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
            <checkboxItem
              v-for="(page, index) in appInfo.app_setting.introPage.pages"
              disable-checkbox
              :key="index"
              :open="openedPageIndexIntro === index"
              @update:open="
                $event
                  ? (openedPageIndexIntro = index)
                  : (openedPageIndexIntro = -1)
              "
              class="w-full h-fit"
              :label="t('page', { page: index + 1 })"
            >
              <template #end="">
                <HeroiconsXMark
                  class="w-8 h-8 text-slate-700 dark:text-slate-200"
                  @click="removePage(index)"
                />
              </template>

              <template #description="">
                {{ openedPageIndexIntro !== index ? t("pageInfo", page) : "" }}
              </template>
              <template #default="">
                <div class="grid grid-cols-12 gap-4">
                  <textInput
                    v-model="page.title"
                    class="col-span-5"
                    label-class="dark:!bg-slate-700"
                    input-class="dark:!bg-slate-700"
                    :placeholder="t('pageTitle')"
                    :label="t('pageTitleHint')"
                  >
                  </textInput>
                  <textInput
                    v-model="page.description"
                    class="col-span-7"
                    label-class="dark:!bg-slate-700"
                    input-class="dark:!bg-slate-700"
                    :placeholder="t('pageDescriptionHint')"
                    :label="t('pageDescription')"
                  >
                  </textInput>

                  <color-picker
                    label-class="dark:!bg-slate-700"
                    input-class="dark:!bg-slate-700"
                    class="col-span-9"
                    v-model="page.background"
                    :label="
                      $t('steps.splash_screeen_setting.select_your_color')
                    "
                  >
                  </color-picker>

                  <FileSelect
                    class="col-span-12 h-40"
                    v-model="page.imageName"
                    :file-name="`pageImage${index}`"
                    :label="$t('steps.splash_screeen_setting.image')"
                  >
                  </FileSelect>
                </div>
              </template>
            </checkboxItem>
          </TransitionGroup>
        </div>

        <button
          v-if="appInfo.app_setting.introPage.enable"
          class="text-gray-900 dark:text-slate-100 rounded-3xl px-2 max-w-40 py-2 text-base font-medium bottom-4 end-4 absolute bg-indigo-500 flex justify-center items-center gap-2 group transition-all duration-500 ease-in-out"
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
          <span class="group-hover:block hidden"> Add more </span>
          <HeroiconsPlus16Solid class="w-8 h-8" />
        </button>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";
import HeroiconsPlus16Solid from "~icons/heroicons/plus-16-solid";
import HeroiconsXMark from "~icons/heroicons/x-mark";
const { appInfo, openedPageIndexIntro } = storeToRefs(useAppSettingStore());

const { t } = useI18n();

const removePage = (index: number) => {
  appInfo.value.app_setting.introPage.pages.splice(index, 1);
  if (appInfo.value.app_setting.introPage.pages.length === 0)
    appInfo.value.app_setting.introPage.enable = false;
};
</script>

<i18n>
  {
  "en":{
    "pageInfo":"Page with title '{title}' and description of '{description}'",
    "pageTitleHint":"Enter the page title",
    "pageTitle":"Page title",
    "pageDescriptionHint":"Enter the page description",
    "pageDescription":"Page description",
    "page":"Page {page}"
  }
}
</i18n>

<template>
  <checkboxItem
    disable-checkbox
    :open="openedPageIndexIntro === index"
    @update:open="
      $event ? (openedPageIndexIntro = index) : (openedPageIndexIntro = -1)
    "
    :error="v$.$error ? t('error') : ''"
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
      {{ openedPageIndexIntro !== index ? t("pageInfo", page as any) : "" }}
    </template>
    <template #default="">
      <div class="grid grid-cols-12 gap-4">
        <textInput
          v-model="page.title"
          class="col-span-5"
          :error="v$.title.$errors.map((e) => e.$message).join(',')"
          label-class="dark:!bg-slate-700"
          input-class="dark:!bg-slate-700"
          :placeholder="t('pageTitle')"
          :label="t('pageTitleHint')"
        >
        </textInput>
        <textInput
          v-model="page.description"
          class="col-span-7"
          :error="v$.description.$errors.map((e) => e.$message).join(',')"
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
          :error="v$.background.$errors.map((e) => e.$message).join(',')"
          :label="$t('steps.splash_screeen_setting.select_your_color')"
        >
        </color-picker>

        <FileSelect
          class="col-span-12 h-40"
          v-model="page.imageName"
          :error="v$.imageName.$errors.map((e) => e.$message).join(',')"
          :file-name="`pageImage${index}`"
          :label="$t('steps.splash_screeen_setting.image')"
        >
        </FileSelect>
      </div>
    </template>
  </checkboxItem>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { IntroPage, useAppSettingStore } from "@/stores/appSetting";
import HeroiconsXMark from "~icons/heroicons/x-mark";
import { storeToRefs } from "pinia";
import useVuelidate from "@vuelidate/core";
import { useI18nValidators } from "@/utils/i18n-validators";
const { required } = useI18nValidators();
const { t } = useI18n();
const { appInfo, openedPageIndexIntro } = storeToRefs(useAppSettingStore());

const props = defineProps<{
  page: IntroPage;
  index: number;
}>();

const removePage = (index: number) => {
  appInfo.value.app_setting.introPage.pages.splice(index, 1);
  if (appInfo.value.app_setting.introPage.pages.length === 0)
    appInfo.value.app_setting.introPage.enable = false;
};

const rules = {
  title: {
    required,
  },
  description: {
    required,
  },
  background: {
    required,
  },
  imageName: {
    required,
  },
};

const v$ = useVuelidate(rules, props.page);
</script>

<i18n>
  {
  "en":{
    "pageInfo":"Page with title '{title}' and description of '{description}'",
    "pageTitleHint":"Enter the page title",
    "pageTitle":"Page title",
    "pageDescriptionHint":"Enter the page description",
    "pageDescription":"Page description",
    "page":"Page {page}",
  "error":"there is error in this item"
  }
}
</i18n>

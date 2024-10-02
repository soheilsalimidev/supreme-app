<template>
  <div class="flex flex-col h-full">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <div class="grid grid-cols-5 gap-6">
        <textInput
          class="col-span-2"
          v-model="appInfo.name"
          :error="v$.name.$errors.map((e: any) => e.$message).join(',')"
          @blur="v$.name.$touch"
          :placeholder="$t('steps.app_info.my_amazing_app')"
          :label="$t('steps.app_info.app_name')"
        >
          ></textInput
        >
        <textInput
          v-model="url_site"
          class="col-span-3"
          :error="
            v$.app_setting.site_url.$errors.map((e) => e.$message).join(',')
          "
          @blur="v$.app_setting.site_url.$touch"
          :placeholder="$t('steps.app_info.https_supermerapp_com')"
          :label="$t('steps.app_info.website')"
        ></textInput>

        <textInput
          v-model="packageName"
          class="col-span-2"
          :error="v$.package_name.$errors.map((e: any) => e.$message).join(',')"
          @blur="v$.package_name.$touch"
          :placeholder="$t('steps.app_info.com_app_webapp')"
          :label="$t('steps.app_info.app_packagename')"
        >
        </textInput>

        <div class="flex gap-2 col-span-3">
          <VSwitch
            v-model="appInfo.colors.metarial"
            :label="
              appInfo.colors.metarial
                ? 'Use Materail 3 auto color choosing'
                : ''
            "
          ></VSwitch>
          <ColorPicker
            v-if="!appInfo.colors.metarial"
            v-model="colorPicker"
            class="grow"
            :gradient="false"
            :error="
              v$.app_setting.site_url.$errors.map((e) => e.$message).join(',')
            "
            label="App priamry color"
          ></ColorPicker>
        </div>
      </div>
    </div>

    <fileSelect
      v-model="appInfo.icon_path"
      class="grow"
      file-name="logo.png"
      filter-warning-title="inviled image"
      filter-warning-text="your image should be 512*512 size"
      :error="v$.icon_path.$errors.map((e: any) => e.$message).join(',')"
      :filter-condition="
        (width: number, height: number) =>
          (width === 512 && height === 512) ||
          (width === 48 && height === 48) ||
          (width === 192 && height === 192)
      "
      :label="$t('steps.app_info.app_icon')"
    >
    </fileSelect>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import useVuelidate from "@vuelidate/core";
import { useI18nValidators } from "@/utils/i18n-validators";
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";
import { helpers } from "@vuelidate/validators";
import { useI18n } from "vue-i18n";
import {
  argbFromRgb,
  themeFromSourceColor,
  hexFromArgb,
} from "@material/material-color-utilities";

// Get the theme from a hex color

const { appInfo } = storeToRefs(useAppSettingStore());
const isPackChangeed = ref(false);
const { t } = useI18n();
const { required, url } = useI18nValidators();

const colorPicker = computed({
  get() {
    return appInfo.value.colors.primary;
  },
  set(value: string) {
    const [r, g, b] = value
      .replace("rgb(", "")
      .replace(")", "")
      .split(",")
      .map((val) => parseInt(val.trim()));
    const theme = themeFromSourceColor(argbFromRgb(r, g, b));
    appInfo.value.colors.primary = value;
    appInfo.value.colors.light = Object.fromEntries(
      Object.entries(
        Object.values(theme.schemes.light)[0] as { [k in string]: number },
      )
        .filter(([k]) =>
          [
            "primary",
            "onPrimary",
            "primaryContainer",
            "onPrimaryContainer",
            "secondary",
            "onSecondary",
            "secondaryContainer",
            "onSecondaryContainer",
            "tertiary",
            "onTertiary",
            "tertiaryContainer",
            "onTertiaryContainer",
            "error",
            "onError",
            "errorContainer",
          ].includes(k),
        )
        .map(([k, v]) => [k, hexFromArgb(v)]),
    );
    appInfo.value.colors.dark = Object.fromEntries(
      Object.entries(
        Object.values(theme.schemes.dark)[0] as { [k in string]: number },
      )
        .filter(([k]) =>
          [
            "primary",
            "onPrimary",
            "primaryContainer",
            "onPrimaryContainer",
            "secondary",
            "onSecondary",
            "secondaryContainer",
            "onSecondaryContainer",
            "tertiary",
            "onTertiary",
            "tertiaryContainer",
            "onTertiaryContainer",
            "error",
            "onError",
            "errorContainer",
          ].includes(k),
        )
        .map(([k, v]) => [k, hexFromArgb(v)]),
    );
  },
});

const packageName = computed({
  get() {
    if (!isPackChangeed.value && /^[a-zA-Z]+$/.test(appInfo.value.name)) {
      return (
        "com.app.webapp." +
        appInfo.value.name
          .trim()
          .toLowerCase()
          .replaceAll("-", ".")
          .split(" ")
          .join("")
      );
    } else {
      return appInfo.value.package_name.startsWith("com.app.webapp.")
        ? appInfo.value.package_name.substring(15)
        : appInfo.value.package_name;
    }
  },
  set(value) {
    isPackChangeed.value = true;
    appInfo.value.package_name = "com.app.webapp." + value;
  },
});

const url_site = computed({
  get() {
    return appInfo.value.app_setting.site_url.replace("https://", "");
  },
  set(value: string) {
    appInfo.value.app_setting.site_url = withHttp(value);
  },
});

const withHttp = (url: string) => {
  return url.replace(
    /^(?:(.*:)?\/\/)?(.*)/i,
    (match, schemma, nonSchemmaUrl) =>
      schemma ? match : `https://${nonSchemmaUrl}`,
  );
};

const rules = {
  package_name: {
    required,
    packname: helpers.withMessage(
      t("packname"),
      helpers.regex(/^[a-z][a-z0-9_]*(\.[a-z0-9_]+)*[a-z0-9_]*$/),
    ),
  },
  name: {
    required,
  },
  icon_path: {
    required,
  },
  app_setting: {
    site_url: {
      required,
      url,
    },
  },
};

const v$ = useVuelidate(rules, appInfo);
</script>

<i18n lang="json">
{
  "en": {
    "packname": "enter valid package name"
  },
  "fa": {
    "packname": "نام بسته معتبر را وارد کنید"
  }
}
</i18n>

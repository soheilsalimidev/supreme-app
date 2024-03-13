<template>
  <div class="flex flex-col">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <div class="grid grid-cols-3 gap-6">
        <textInput
          v-model="appInfo.name"
          label="App name"
          placeholder="my amazing app"
          :error="v$.name.$errors.map((e) => e.$message).join(',')"
          @blur="v$.name.$touch"
        />
        <textInput
          v-model="packageName"
          class="col-span-2"
          label="App packagename"
          placeholder="com.app.webapp"
          :error="v$.package_name.$errors.map((e) => e.$message).join(',')"
          @blur="v$.package_name.$touch"
        />
        <textInput
          v-model="url_site"
          class="col-span-2"
          label="website"
          placeholder="https://supermerapp.com"
          :error="
            v$.app_setting.site_url.$errors.map((e) => e.$message).join(',')
          "
          @blur="v$.app_setting.site_url.$touch"
        />
      </div>
    </div>

    <fileSelect
      v-model="appInfo.icon_path"
      label="App icon"
      file-name="logo.png"
      filter-warning-title="inviled image"
      filter-warning-text="your image should be 512*512 size"
      :error="v$.package_name.$errors.map((e) => e.$message).join(',')"
      :filter-condition="
        (width: number, height: number) =>
          (width === 512 && height === 512) ||
          (width === 48 && height === 48) ||
          (width === 192 && height === 192)
      "
    />
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import useVuelidate from "@vuelidate/core";
import { required, helpers, url } from "@vuelidate/validators";
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";
const { appInfo } = storeToRefs(useAppSettingStore());
const isPackChangeed = ref(false);

const packageName = computed({
  get() {
    if (!isPackChangeed.value) {
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
      return appInfo.value.package_name.substring(15);
    }
  },
  set(value) {
    isPackChangeed.value = true;
    appInfo.value.package_name = "com.app.webapp." + value;
  },
});

const url_site = computed({
  get() {
    return appInfo.value.app_setting.site_url;
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
      "enter valid package name",
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

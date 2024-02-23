<template>
  <div class="flex flex-col">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <div class="grid grid-cols-3 gap-6">
        <textInput
          label="App name"
          placeholder="my amazing app"
          :error="v$.name.$errors.map((e) => e.$message).join(',')"
          @blur="v$.name.$touch"
          v-model="appInfo.name"
        />
        <textInput
          class="col-span-2"
          label="App packagename"
          placeholder="com.app.webapp"
          @blur="v$.package_name.$touch"
          :error="v$.package_name.$errors.map((e) => e.$message).join(',')"
          v-model="appInfo.package_name"
        />
        <textInput
          class="col-span-2"
          label="website"
          placeholder="supermerapp.com"
          @blur="v$.app_setting.site_url.$touch"
          :error="
            v$.app_setting.site_url.$errors.map((e) => e.$message).join(',')
          "
          v-model="appInfo.app_setting.site_url"
        />
      </div>
    </div>

    <fileSelect
      label="App icon"
      key="logo.png"
      v-model="appInfo.icon_path"
      :error="v$.package_name.$errors.map((e) => e.$message).join(',')"
      :filterCondition="
        (width: number, height: number) =>
          (width === 512 && height === 512) ||
          (width === 48 && height === 48) ||
          (width === 192 && height === 192)
      "
    ></fileSelect>

    <div class="px-4 py-3 text-right sm:px-6 mt-auto">
      <button
        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        @click="next"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import useVuelidate from "@vuelidate/core";
import { required, helpers, url } from "@vuelidate/validators";
import { storeToRefs } from "pinia";
import { watch } from "vue";
import { useNavigationStore } from "@/stores/navigation";

const { steps, activeTabIndex } = storeToRefs(useNavigationStore());
const { appInfo } = storeToRefs(useAppSettingStore());

watch(
  () => appInfo.value.name,
  () =>
    (appInfo.value.package_name = appInfo.value.name
      .trim()
      .toLowerCase()
      .replaceAll("-", ".")
      .split(" ")
      .join(".")),
);

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

const next = async () => {
  if (await v$.value.$validate()) {
    steps.value[0].status = "complete";
    steps.value[1].status = "current";
    activeTabIndex.value++;
  }
};
</script>

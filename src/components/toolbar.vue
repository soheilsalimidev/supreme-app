<template>
  <div class="flex w-full">
    <div
      data-tauri-drag-region
      class="h-6 select-none flex justify-end w-full bg-indigo-700 px-2 gap-3 items-center"
    >
      <VMenu name="Files" :menuItems="fileMenu"></VMenu>
      <VMenu name="Settings" :menuItems="settingMenu"></VMenu>
      <div class="flex gap-2 ms-auto">
        <div
          class="inline-flex justify-center items-center h-4 w-4 bg-amber-700 rounded-full group hover:scale-125 transition"
          @click="appWindow.minimize()"
        >
          <SolarMinusCircleBroken
            class="text-orange-200 p-[2px] hidden group-hover:block"
          />
        </div>

        <div
          class="inline-flex justify-center items-center h-4 w-4 bg-blue-600 rounded-full group hover:scale-125 transition"
          @click="
            () => {
              appWindow.toggleMaximize();
              isMax = !isMax;
            }
          "
        >
          <component
            :is="!isMax ? SolarMaximizeBroken : SolarMinimizeBroken"
            class="text-blue-100 p-[2px] hidden group-hover:block"
          />
        </div>
        <div
          class="inline-flex justify-center items-center h-4 w-4 bg-red-800 rounded-full group hover:scale-125 transition"
          @click="checkBeforeTheClose"
        >
          <SolarCloseCircleBroken
            class="text-red-100 p-[1px] hidden group-hover:block"
          />
        </div>
      </div>
    </div>
  </div>
  <modal
    v-model="openSaveModel"
    color="error"
    ok-text="Exit anyway"
    cancel-text="Save"
    title="You have unsaved changes !"
    :cancel="saveAppAsFile"
    :ok="appWindow.close"
  >
    <template #default>
      You Have unsaved changes, if you exit without saving you're going to lose
      it all
    </template>
    <template #icon>
      <LineMdAlertLoop />
    </template>
  </modal>
</template>

<script lang="ts" setup>
import { getCurrentWindow } from "@tauri-apps/api/window";
import SolarCloseCircleBroken from "~icons/solar/close-circle-broken";
import SolarMaximizeBroken from "~icons/solar/maximize-broken";
import SolarMinusCircleBroken from "~icons/solar/minus-circle-broken";
import SolarMinimizeBroken from "~icons/solar/minimize-broken";
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import { ref } from "vue";
import { importFromTheAppFile, saveAppAsFile } from "@/utils/save";
import { useDark, useToggle } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import { useAppSettingStore } from "@/stores/appSetting";

const openSaveModel = ref(false);
const appDate = useAppSettingStore();
const { locale } = useI18n({ useScope: "global" });
const isDark = useDark();
const toggleDark = useToggle(isDark);
const langs = {
  en: "تعییر زبان به فارسی",
  fa: "Change language to English",
};

const fileMenu = [
  { name: "save", click: saveAppAsFile },
  { name: "import", click: importFromTheAppFile },
];

const changeLocal = () => {
  locale.value = locale.value === "fa" ? "en" : "fa";
  document.dir = locale.value === "fa" ? "rtl" : "lrt";
  document.documentElement.lang = locale.value;
  settingMenu.value[1].name = langs[locale.value as keyof typeof langs];
};

const settingMenu = ref<
  {
    name: string;
    click: () => void;
  }[]
>([
  {
    name: `change theme to ${!isDark.value ? "dark" : "light"}`,
    click: () => {
      toggleDark();
      settingMenu.value[0].name = `change theme to ${!isDark.value ? "dark" : "light"}`;
    },
  },
  {
    name: langs[locale.value as keyof typeof langs],
    click: changeLocal,
  },
]);

const isMax = ref(false);
const appWindow = getCurrentWindow();
appWindow.listen("tauri://resize", async () => {
  isMax.value = await appWindow.isMaximized();
});

const checkBeforeTheClose = async () => {
  if (JSON.stringify(appDate.appInfo) === appDate.lastUpdateFile) {
    await appWindow.close();
    return;
  }
  openSaveModel.value = true;
};
</script>

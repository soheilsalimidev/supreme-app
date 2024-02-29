<template>
  <div class="flex gap-4 flex-col w-full">
    <checkboxItem
      :open="items[0]"
      @update:open="openNew(0)"
      label="swipe_refresh"
      v-model="appInfo.app_setting.swipe_refresh"
    >
      <template #description> you can have Swipe Refresh page </template>
    </checkboxItem>
    <checkboxItem
      label="cache_mode"
      disableCheckbox
      :open="items[1]"
      @update:open="openNew(1)"
    >
      <template #description>
        There are multi type of cacheMode you can select
      </template>

      <template #default>
        <radioList
          :items="cacheModes"
          :default-value="1"
          label="sidebar_menu_header_color"
          v-model="appInfo.app_setting.cache_mode"
        ></radioList>
      </template>
    </checkboxItem>
    <checkboxItem
      label="no_internet_layout"
      disableCheckbox
      @update:open="openNew(2)"
      :open="items[2]"
    >
      <template #description> There are multi layout of no Internet</template>
      <template #default>
        <radioList
          :items="noNetLayouts"
          :default-value="1"
          label="no internet layout"
          v-model="appInfo.app_setting.no_internet_layout.type"
        ></radioList>

        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 1"
          class="mt-2"
        >
          <fileSelect
            file-name="loading.json"
            label="select your lottie file"
            :accept="['json']"
            v-model="appInfo.app_setting.no_internet_layout.lottieFile"
          ></fileSelect>
        </div>
        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 2"
          class="mt-2"
        >
          <fileSelect
            file-name="no_internet.json"
            label="select your image"
            v-model="appInfo.app_setting.no_internet_layout.image"
          ></fileSelect>
        </div>
        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 m-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <ExclamationTriangleIcon
                class="h-5 w-5 text-yellow-400"
                aria-hidden="true"
              />
            </div>
            <div class="ml-3">
              <p class="text-sm text-yellow-700">
                if you don't select anything
                {{ " " }}
                <a
                  href="#"
                  class="font-medium underline text-yellow-700 hover:text-yellow-600"
                >
                  it will you the default one for you
                </a>
              </p>
            </div>
          </div>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      label="toolbar"
      disableCheckbox
      @update:open="openNew(3)"
      :open="items[3]"
      :error="v$.app_setting.toolbar.$errors.map((e) => e.$message).join(',')"
    >
      <template #description> You can have different type of toolbar </template>
      <template #default>
        <radioList
          :items="toolbarsModes"
          :default-value="0"
          label="You can have different type of toolbar"
          v-model="appInfo.app_setting.toolbar.type"
        ></radioList>

        <div v-if="appInfo.app_setting.toolbar.type === 1" class="mt-4">
          <textInput
            label="the text to be dispalyed"
            :value="appInfo.name"
            placeholder="my amazing app"
            labelClass="dark:!bg-slate-700"
            inputClass="dark:!bg-slate-700"
            :error="
              v$.app_setting.toolbar.text.$errors
                .map((e) => e.$message)
                .join(',')
            "
            v-model="appInfo.app_setting.toolbar.text"
          ></textInput>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      :open="items[4]"
      label="toolbar_custom_icon"
      @update:open="openNew(4)"
      v-model="appInfo.app_setting.toolbar_custom_icon.enable"
      :error="
        v$.app_setting.toolbar_custom_icon.$errors
          .map((e) => e.$message)
          .join(',')
      "
    >
      <template #description>
        You can have icon that show url in web view when you click on it
      </template>
      <template #default>
        <div class="gap-6 flex flex-col">
          <fileSelect
            file-name="toolbar_icon.png"
            :error="
              v$.app_setting.toolbar_custom_icon.first.$errors
                .map((e) => e.$message)
                .join(',')
            "
            label="icon to be dispalyed"
            v-model="appInfo.app_setting.toolbar_custom_icon.first"
          ></fileSelect>

          <textInput
            label="link to be open"
            :value="appInfo.name"
            :error="
              v$.app_setting.toolbar_custom_icon.second.$errors
                .map((e) => e.$message)
                .join(',')
            "
            placeholder="myapp.com"
            labelClass="dark:!bg-slate-700"
            inputClass="dark:!bg-slate-700"
            v-model="appInfo.app_setting.toolbar_custom_icon.second"
          ></textInput>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      label="sidebar_menu"
      :open="items[5]"
      @update:open="openNew(5)"
      v-model="appInfo.app_setting.sidebar_menu.enable"
      :error="
        v$.app_setting.sidebar_menu.$errors.map((e) => e.$message).join(',')
      "
    >
      <template #description>
        you can have sidebar menu (if you want to have menu its better to have
        toolbar too)
      </template>
      <template #default>
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">sidebar_menu_header_color</h2>
          <radioList
            :items="sidebarMenuHeaderModes"
            :default-value="0"
            label="sidebar_menu_header_color"
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_header.type"
          ></radioList>

          <div
            v-if="
              appInfo.app_setting.sidebar_menu.sidebar_menu_header.type === 1
            "
          >
            <color-picker
              v-model:gradientColor="
                appInfo.app_setting.sidebar_menu.sidebar_menu_header.color
              "
              lang="En"
              :theme="isDark ? 'black' : 'white'"
              useType="gradient"
            />
            <p class="mt-2 text-sm text-red-600">
              {{
                v$.app_setting.sidebar_menu.sidebar_menu_header.color.$errors
                  .map((e) => e.$message)
                  .join(",")
              }}
            </p>
          </div>
          <h2 class="dark:text-white text-lg">sidebar_menu_footer</h2>
          <radioList
            :items="sidebarMenuFooterModes"
            :default-value="0"
            label="you can change footer on slider"
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_footer.type"
          ></radioList>
          <div>
            <textInput
              label="the text to be dispalyed"
              placeholder="my amazing app"
              labelClass="dark:!bg-slate-700"
              inputClass="dark:!bg-slate-700"
              :error="
                v$.app_setting.sidebar_menu.sidebar_menu_footer.text.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
              v-model="
                appInfo.app_setting.sidebar_menu.sidebar_menu_footer.text
              "
            ></textInput>
          </div>
          <h2 class="dark:text-white text-lg">items</h2>
          <ListItemMaker
            v-model="appInfo.app_setting.sidebar_menu.item_menu"
          ></ListItemMaker>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      :open="items[6]"
      label="floating_action_button"
      v-model="appInfo.app_setting.floating_action_button.enable"
      @update:open="openNew(6)"
      :error="
        v$.app_setting.floating_action_button.$errors
          .map((e) => e.$message)
          .join(',')
      "
    >
      <template #description> floating action button menu </template>
      <template #default>
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">items</h2>
          <ListItemMaker
            v-model="appInfo.app_setting.floating_action_button.item_fab"
          ></ListItemMaker>
        </div>
      </template>
    </checkboxItem>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import ExclamationTriangleIcon from '~icons/heroicons/exclamation-triangle';
import useVuelidate from "@vuelidate/core";
import { requiredIf } from "@vuelidate/validators";
import { storeToRefs } from "pinia";
import { ColorPicker } from "vue3-colorpicker";
import "vue3-colorpicker/style.css";
import { useDark } from "@vueuse/core";
import { ref } from "vue";

const items = ref([false, false, false, false, false, false, false]);

const isDark = useDark();

const { appInfo } = storeToRefs(useAppSettingStore());

const openNew = (index: number) => {
  if (items.value[index]) {
    items.value[index] = false;
  } else {
    items.value.forEach((_, index) => (items.value[index] = false));
    items.value[index] = true;
  }
};

const rules = {
  app_setting: {
    toolbar: {
      text: {
        req: requiredIf(() => appInfo.value.app_setting.toolbar.type === 1),
      },
    },
    toolbar_custom_icon: {
      first: {
        req: requiredIf(
          () => appInfo.value.app_setting.toolbar_custom_icon.enable,
        ),
      },
      second: {
        req: requiredIf(
          () => appInfo.value.app_setting.toolbar_custom_icon.enable,
        ),
      },
    },
    floating_action_button: {
      item_fab: {
        req: requiredIf(
          () => appInfo.value.app_setting.floating_action_button.enable,
        ),
      },
    },
    sidebar_menu: {
      sidebar_menu_header: {
        color: {
          req: requiredIf(
            () =>
              appInfo.value.app_setting.sidebar_menu.sidebar_menu_header
                .type === 1,
          ),
        },
      },
      sidebar_menu_footer: {
        text: {
          req: requiredIf(
            () =>
              appInfo.value.app_setting.sidebar_menu.sidebar_menu_footer
                .type === 1,
          ),
        },
      },
    },
  },
};

const v$ = useVuelidate(rules, appInfo);

const sidebarMenuFooterModes = [
  {
    title: "no footer",
    description: "",
    value: 0,
  },
  {
    title: "with footer",
    description: "",
    value: 1,
  },
];

const sidebarMenuHeaderModes = [
  {
    title: "no header",
    description: "",
    value: 0,
  },
  {
    title: "header with gradient",
    description: "",
    value: 1,
  },
];

const noNetLayouts = [
  {
    title: "simple with image and a text",
    description: "",
    value: 1,
  },
  {
    title: "with lottie animation and a text",
    description: "",
    value: 2,
  },
];

const toolbarsModes = [
  {
    title: "no toolbar",
    description: "",
    value: 0,
  },
  {
    title: "toolbar with text",
    description: "",
    value: 1,
  },
];

const cacheModes = [
  {
    title: "LOAD_DEFAULT",
    description:
      "Default cache usage mode. If the navigation type doesn't impose any specific behavior, use cached resources when they are available and not expired, otherwise load resources from the network",
    value: 1,
  },
  {
    title: "LOAD CACHE ELSE NETWORK",
    description:
      "Use cached resources when they are available, even if they have expired. Otherwise load resources from the network.",
    value: 2,
  },
];
</script>

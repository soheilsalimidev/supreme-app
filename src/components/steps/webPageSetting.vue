<template>
  <div class="flex gap-4 flex-col w-full">
    <checkboxItem
      v-model="appInfo.app_setting.swipe_refresh"
      :open="items[0]"
      label="swipe_refresh"
      @update:open="openNew(0)"
    >
      <template #description>
        you can have Swipe Refresh page
      </template>
    </checkboxItem>
    <checkboxItem
      label="cache_mode"
      disable-checkbox
      :open="items[1]"
      @update:open="openNew(1)"
    >
      <template #description>
        There are multi type of cacheMode you can select
      </template>

      <template #default>
        <radioList
          v-model="appInfo.app_setting.cache_mode"
          :items="cacheModes"
          :default-value="1"
          label="sidebar_menu_header_color"
        />
      </template>
    </checkboxItem>
    <checkboxItem
      label="no_internet_layout"
      disable-checkbox
      :open="items[2]"
      @update:open="openNew(2)"
    >
      <template #description>
        There are multi layout of no Internet
      </template>
      <template #default>
        <radioList
          v-model="appInfo.app_setting.no_internet_layout.type"
          :items="noNetLayouts"
          :default-value="1"
          label="no internet layout"
        />

        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 2"
          class="mt-2"
        >
          <fileSelect
            v-model="appInfo.app_setting.no_internet_layout.lottieFile"
            file-name="no_internet.json"
            label="select your lottie file"
            :accept="['json']"
          />
        </div>
        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 1"
          class="mt-2"
        >
          <fileSelect
            v-model="appInfo.app_setting.no_internet_layout.image"
            file-name="no_internet.json"
            label="select your image"
          />
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
      disable-checkbox
      :open="items[3]"
      :error="v$.app_setting.toolbar.$errors.map((e) => e.$message).join(',')"
      @update:open="openNew(3)"
    >
      <template #description>
        You can have different type of toolbar
      </template>
      <template #default>
        <radioList
          v-model="appInfo.app_setting.toolbar.type"
          :items="toolbarsModes"
          :default-value="0"
          label="You can have different type of toolbar"
        />

        <div
          v-if="appInfo.app_setting.toolbar.type === 1"
          class="mt-4"
        >
          <textInput
            v-model="appInfo.app_setting.toolbar.text"
            label="the text to be dispalyed"
            :value="appInfo.name"
            placeholder="my amazing app"
            label-class="dark:!bg-slate-700"
            input-class="dark:!bg-slate-700"
            :error="
              v$.app_setting.toolbar.text.$errors
                .map((e) => e.$message)
                .join(',')
            "
          />
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-if="appInfo.app_setting.toolbar.type === 1"
      v-model="appInfo.app_setting.toolbar_custom_icon.enable"
      :open="items[4]"
      label="toolbar_custom_icon"
      :error="
        v$.app_setting.toolbar_custom_icon.$errors
          .map((e) => e.$message)
          .join(',')
      "
      @update:open="openNew(4)"
    >
      <template #description>
        You can have icon that show url in web view when you click on it
      </template>
      <template #default>
        <div class="gap-6 flex flex-col">
          <fileSelect
            v-model="appInfo.app_setting.toolbar_custom_icon.first"
            file-name="toolbar_icon.png"
            :error="
              v$.app_setting.toolbar_custom_icon.first.$errors
                .map((e) => e.$message)
                .join(',')
            "
            label="icon to be dispalyed"
          />

          <textInput
            v-model="appInfo.app_setting.toolbar_custom_icon.second"
            label="link to be open"
            :value="appInfo.name"
            :error="
              v$.app_setting.toolbar_custom_icon.second.$errors
                .map((e) => e.$message)
                .join(',')
            "
            placeholder="myapp.com"
            label-class="dark:!bg-slate-700"
            input-class="dark:!bg-slate-700"
          />
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-model="appInfo.app_setting.sidebar_menu.enable"
      label="sidebar_menu"
      :open="items[5]"
      :error="
        v$.app_setting.sidebar_menu.$errors.map((e) => e.$message).join(',')
      "
      @update:open="openNew(5)"
    >
      <template #description>
        you can have sidebar menu (if you want to have menu its better to have
        toolbar too)
      </template>
      <template #default>
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">
            sidebar_menu_header_color
          </h2>
          <radioList
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_header.type"
            :items="sidebarMenuHeaderModes"
            :default-value="0"
            label="sidebar_menu_header_color"
          />

          <div
            v-if="
              appInfo.app_setting.sidebar_menu.sidebar_menu_header.type === 1
            "
          >
            <color-picker
              v-model="
                appInfo.app_setting.sidebar_menu.sidebar_menu_header.color
              "
              label="select your colors"
            />
            <p class="mt-2 text-sm text-red-600">
              {{
                v$.app_setting.sidebar_menu.sidebar_menu_header.color.$errors
                  .map((e) => e.$message)
                  .join(",")
              }}
            </p>
          </div>
          <h2 class="dark:text-white text-lg">
            sidebar_menu_footer
          </h2>
          <radioList
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_footer.type"
            :items="sidebarMenuFooterModes"
            :default-value="0"
            label="you can change footer on slider"
          />
          <div>
            <textInput
              v-model="
                appInfo.app_setting.sidebar_menu.sidebar_menu_footer.text
              "
              label="the text to be dispalyed"
              placeholder="my amazing app"
              label-class="dark:!bg-slate-700"
              input-class="dark:!bg-slate-700"
              :error="
                v$.app_setting.sidebar_menu.sidebar_menu_footer.text.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
            />
          </div>
          <h2 class="dark:text-white text-lg">
            items
          </h2>
          <ListItemMaker
            v-model="appInfo.app_setting.sidebar_menu.item_menu"
          />
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-model="appInfo.app_setting.floating_action_button.enable"
      :open="items[6]"
      label="floating_action_button"
      :error="
        v$.app_setting.floating_action_button.$errors
          .map((e) => e.$message)
          .join(',')
      "
      @update:open="openNew(6)"
    >
      <template #description>
        floating action button menu
      </template>
      <template #default>
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">
            items
          </h2>
          <ListItemMaker
            v-model="appInfo.app_setting.floating_action_button.item_fab"
          />
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      :open="items[7]"
      label="loading"
      disable-checkbox
      @update:open="openNew(7)"
    >
      <template #description>
        loading lottie
      </template>
      <template #default>
        <fileSelect
          v-model="appInfo.app_setting.loading"
          file-name="loading.json"
          label="select your lottie file"
          :accept="['json']"
        />

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
                  the default one will be selected
                </a>
              </p>
            </div>
          </div>
        </div>
      </template>
    </checkboxItem>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import ExclamationTriangleIcon from "~icons/heroicons/exclamation-triangle";
import useVuelidate from "@vuelidate/core";
import { requiredIf } from "@vuelidate/validators";
import { storeToRefs } from "pinia";

const { appInfo, selectedWebPageSetting: items } = storeToRefs(
  useAppSettingStore()
);

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
          () => appInfo.value.app_setting.toolbar_custom_icon.enable
        ),
      },
      second: {
        req: requiredIf(
          () => appInfo.value.app_setting.toolbar_custom_icon.enable
        ),
      },
    },
    floating_action_button: {
      item_fab: {
        req: requiredIf(
          () => appInfo.value.app_setting.floating_action_button.enable
        ),
      },
    },
    sidebar_menu: {
      sidebar_menu_header: {
        color: {
          req: requiredIf(
            () =>
              appInfo.value.app_setting.sidebar_menu.sidebar_menu_header
                .type === 1
          ),
        },
      },
      sidebar_menu_footer: {
        text: {
          req: requiredIf(
            () =>
              appInfo.value.app_setting.sidebar_menu.sidebar_menu_footer
                .type === 1
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

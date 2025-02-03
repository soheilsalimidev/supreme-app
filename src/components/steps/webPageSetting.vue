<template>
  <div class="flex gap-4 flex-col w-full">
    <checkboxItem
      v-model="appInfo.app_setting.swipe_refresh"
      :open="items[0]"
      :label="$t('steps.web_page_setting.swipe_refresh')"
      @update:open="openNew(0)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.you_can_have_swipe_refresh") }}
      </template>
    </checkboxItem>
    <checkboxItem
      disable-checkbox
      :open="items[1]"
      :label="$t('steps.web_page_setting.cache_mode')"
      @update:open="openNew(1)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.there_are_multi_type_of_cache_mode") }}
      </template>

      <template #default>
        <radioList
          v-model="appInfo.app_setting.cache_mode"
          :items="cacheModes"
          :default-value="1"
          :label="$t('steps.web_page_setting.sidebar_menu_header_color')"
        >
        </radioList
      ></template>
    </checkboxItem>
    <checkboxItem
      disable-checkbox
      :open="items[2]"
      :label="$t('steps.web_page_setting.no_internet_layout')"
      @update:open="openNew(2)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.there_are_multi_layout_of_no_internet") }}
      </template>
      <template #default="">
        <radioList
          v-model="appInfo.app_setting.no_internet_layout.type"
          :items="noNetLayouts"
          :default-value="1"
          :label="$t('steps.web_page_setting.no_internet_layout2')"
        >
        </radioList>

        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 2"
          class="mt-2"
        >
          <fileSelect
            v-model="appInfo.app_setting.no_internet_layout.lottieFile"
            file-name="no_internet.json"
            :accept="['json']"
            :label="$t('steps.web_page_setting.select_your_lottie_file')"
          >
          </fileSelect>
        </div>
        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 1"
          class="mt-2"
        >
          <fileSelect
            v-model="appInfo.app_setting.no_internet_layout.image"
            file-name="no_internet.json"
            :label="$t('steps.web_page_setting.select_your_image')"
          >
          </fileSelect>
        </div>
        <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 m-4">
          <div class="flex">
            <div class="shrink-0">
              <ExclamationTriangleIcon
                class="h-5 w-5 text-yellow-400"
                aria-hidden="true"
              >
              </ExclamationTriangleIcon>
            </div>
            <div class="ml-3">
              <p class="text-sm text-yellow-700">
                {{ $t("steps.web_page_setting.if_you_don_t_select_anything")
                }}<a
                  href="#"
                  class="font-medium underline text-yellow-700 hover:text-yellow-600"
                >
                  {{ $t("steps.web_page_setting.it_will_you_the_default_one_for_you") }}
                </a>
              </p>
            </div>
          </div>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      disable-checkbox
      :open="items[3]"
      :error="v$.app_setting.toolbar.$errors.map((e) => e.$message).join(',')"
      :label="$t('steps.web_page_setting.toolbar')"
      @update:open="openNew(3)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.you_can_have_different_type_of_toolbar") }}
      </template>
      <template #default="">
        <radioList
          v-model="appInfo.app_setting.toolbar.type"
          :items="toolbarsModes"
          :default-value="0"
          :label="$t('steps.web_page_setting.you_can_have_different_type_of_toolbar')"
        >
        </radioList>
        <div v-if="appInfo.app_setting.toolbar.type === 1" class="mt-4">
          <textInput
            v-model="appInfo.app_setting.toolbar.text"
            :value="appInfo.name"
            label-class="dark:bg-slate-700!"
            input-class="dark:bg-slate-700!"
            :error="
              v$.app_setting.toolbar.text.$errors
                .map((e) => e.$message)
                .join(',')
            "
            :placeholder="$t('steps.web_page_setting.my_amazing_app')"
            :label="$t('steps.web_page_setting.the_text_to_be_dispalyed')"
          >
          </textInput>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-if="appInfo.app_setting.toolbar.type === 1"
      v-model="appInfo.app_setting.toolbar_custom_icon.enable"
      :open="items[4]"
      :error="
        v$.app_setting.toolbar_custom_icon.$errors
          .map((e) => e.$message)
          .join(',')
      "
      :label="$t('steps.web_page_setting.toolbar_custom_icon')"
      @update:open="openNew(4)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.you_can_have_icon_that_show_url_in_web_view") }}
      </template>
      <template #default="">
        <div class="gap-6 flex flex-col">
          <fileSelect
            v-model="appInfo.app_setting.toolbar_custom_icon.first"
            file-name="toolbar_icon.png"
            :error="
              v$.app_setting.toolbar_custom_icon.first.$errors
                .map((e) => e.$message)
                .join(',')
            "
            :label="$t('steps.web_page_setting.icon_to_be_dispalyed')"
          >
            <textInput
              v-model="appInfo.app_setting.toolbar_custom_icon.second"
              :value="appInfo.name"
              :error="
                v$.app_setting.toolbar_custom_icon.second.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
              label-class="dark:bg-slate-700!"
              input-class="dark:bg-slate-700!"
              :placeholder="$t('steps.web_page_setting.myapp_com')"
              :label="$t('steps.web_page_setting.link_to_be_open')"
            >
            </textInput
          ></fileSelect>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-model="appInfo.app_setting.sidebar_menu.enable"
      :open="items[5]"
      :error="
        v$.app_setting.sidebar_menu.$errors.map((e) => e.$message).join(',')
      "
      :label="$t('steps.web_page_setting.sidebar_menu')"
      @update:open="openNew(5)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.you_can_have_sidebar_menu") }}</template
      >
      <template #default="">
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">
            {{ $t("steps.web_page_setting.sidebar_menu_header_color") }}
          </h2>
          <radioList
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_header.type"
            :items="sidebarMenuHeaderModes"
            :default-value="0"
            :label="$t('steps.web_page_setting.sidebar_menu_header_color')"
          >
          </radioList>
          <div
            v-if="
              appInfo.app_setting.sidebar_menu.sidebar_menu_header.type === 1
            "
          >
            <color-picker
              v-model="
                appInfo.app_setting.sidebar_menu.sidebar_menu_header.color!
              "
              :label="$t('steps.web_page_setting.select_your_colors')"
            >
              <p class="mt-2 text-sm text-red-600">
                {{
                  v$.app_setting.sidebar_menu.sidebar_menu_header.color.$errors
                    .map((e) => e.$message)
                    .join(",")
                }}
              </p>
            </color-picker>
          </div>
          <h2 class="dark:text-white text-lg">
            {{ $t("steps.web_page_setting.sidebar_menu_footer") }}
          </h2>
          <radioList
            v-model="appInfo.app_setting.sidebar_menu.sidebar_menu_footer.type"
            :items="sidebarMenuFooterModes"
            :default-value="0"
            :label="$t('steps.web_page_setting.you_can_change_footer_on_slider')"
          >
          </radioList>
          <div>
            <textInput
              v-if="
                appInfo.app_setting.sidebar_menu.sidebar_menu_footer.type === 1
              "
              v-model="
                appInfo.app_setting.sidebar_menu.sidebar_menu_footer.text
              "
              label-class="dark:bg-slate-700!"
              input-class="dark:bg-slate-700!"
              :error="
                v$.app_setting.sidebar_menu.sidebar_menu_footer.text.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
              :placeholder="$t('steps.web_page_setting.my_amazing_app')"
              :label="$t('steps.web_page_setting.the_text_to_be_dispalyed')"
            >
            </textInput>
          </div>
          <h2 class="dark:text-white text-lg">
            {{ $t("steps.web_page_setting.items") }}
          </h2>
          <ListItemMaker v-model="appInfo.app_setting.sidebar_menu.item_menu" />
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      v-model="appInfo.app_setting.floating_action_button.enable"
      :open="items[6]"
      :error="
        v$.app_setting.floating_action_button.$errors
          .map((e) => e.$message)
          .join(',')
      "
      :label="$t('steps.web_page_setting.floating_action_button')"
      @update:open="openNew(6)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.floating_action_button_menu") }}
      </template>
      <template #default="">
        <div class="flex flex-col gap-6">
          <h2 class="dark:text-white text-lg">
            {{ $t("steps.web_page_setting.items") }}
          </h2>
          <ListItemMaker
            v-model="appInfo.app_setting.floating_action_button.item_fab"
          >
          </ListItemMaker>
        </div>
      </template>
    </checkboxItem>
    <checkboxItem
      :open="items[7]"
      disable-checkbox
      :label="$t('steps.web_page_setting.loading')"
      @update:open="openNew(7)"
    >
      <template #description="">
        {{ $t("steps.web_page_setting.loading_lottie") }}
      </template>
      <template #default="">
        <fileSelect
          v-model="appInfo.app_setting.loading"
          file-name="loading.json"
          :accept="['json']"
          :label="$t('steps.web_page_setting.select_your_lottie_file')"
        >
          <div class="bg-yellow-50 border-l-4 border-yellow-400 p-4 m-4">
            <div class="flex">
              <div class="shrink-0">
                <ExclamationTriangleIcon
                  class="h-5 w-5 text-yellow-400"
                  aria-hidden="true"
                >
                </ExclamationTriangleIcon>
              </div>
              <div class="ml-3">
                <p class="text-sm text-yellow-700">
                  {{ $t("steps.web_page_setting.if_you_don_t_select_anything")
                  }}<a
                    href="#"
                    class="font-medium underline text-yellow-700 hover:text-yellow-600"
                  >
                    {{
                      $t("steps.web_page_setting.the_default_one_will_be_se")
                    }}
                  </a>
                </p>
              </div>
            </div>
          </div>
        </fileSelect></template
      >
    </checkboxItem>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import ExclamationTriangleIcon from "~icons/heroicons/exclamation-triangle";
import useVuelidate from "@vuelidate/core";
import { storeToRefs } from "pinia";
import { useI18nValidators } from "@/utils/i18n-validators";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const { appInfo, selectedWebPageSetting: items } =
  storeToRefs(useAppSettingStore());

const { requiredIf } = useI18nValidators();
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
    title: t("sidebarMenuFooterModes.noFooter"),
    description: "",
    value: 0,
  },
  {
    title: t("sidebarMenuFooterModes.withFooter"),
    description: "",
    value: 1,
  },
];

const sidebarMenuHeaderModes = [
  {
    title: t("sidebarMenuHeaderModes.noHeader"),
    description: "",
    value: 0,
  },
  {
    title: t("sidebarMenuHeaderModes.headerWithGradient"),
    description: "",
    value: 1,
  },
];

const noNetLayouts = [
  {
    title: t("noNetLayouts.simpleWithImageAndAText"),
    description: "",
    value: 1,
  },
  {
    title: t("noNetLayouts.withLottieAnimationAndAText"),
    description: "",
    value: 2,
  },
];

const toolbarsModes = [
  {
    title: t("toolbarsModes.noToolbar"),
    description: "",
    value: 0,
  },
  {
    title: t("toolbarsModes.toolbarWithText"),
    description: "",
    value: 1,
  },
];

const cacheModes = [
  {
    title: t("cacheModes.loadDefault"),
    description: t("cacheModes.defaultCacheUsageMode"),
    value: 1,
  },
  {
    title: t("cacheModes.loadCacheElseNetwork"),
    description: t("cacheModes.useCachedResourcesWhenTheyAreAvailable"),
    value: 2,
  },
];

useVuelidate({})

</script>


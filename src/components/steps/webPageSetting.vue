<template>
  <div class="flex gap-4 flex-col w-full overflow-scroll">
    <checkboxItem
      label="swipe_refresh"
      v-model="appInfo.app_setting.swipe_refresh"
    >
      <template #description> you can have Swipe Refresh page </template>
    </checkboxItem>
    <checkboxItem
      label="cache_mode"
      v-model="appInfo.app_setting.cache_mode"
      disableCheckbox
    >
      <template #description>
        There are multi type of cacheMode you can select
      </template>

      <template #default>
        <fieldset>
          <legend class="sr-only">no internet layout</legend>
          <div class="space-y-5">
            <div
              v-for="cacheMode in cacheModes"
              :key="cacheMode.value"
              class="relative flex items-start"
            >
              <div class="flex items-center h-5">
                <input
                  v-model="appInfo.app_setting.no_internet_layout.type"
                  :id="cacheMode.title.toString()"
                  :aria-describedby="`${cacheMode.value}-description`"
                  name="noNet"
                  type="radio"
                  :value="cacheMode.value"
                  :checked="cacheMode.value === 1"
                  class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300"
                />
              </div>
              <div class="ml-3 text-sm">
                <label
                  :for="cacheMode.title.toString()"
                  class="font-medium text-gray-700 dark:text-slate-300"
                  >{{ cacheMode.title }}</label
                >
                <p :id="`${cacheMode.value}-description`" class="text-gray-500">
                  {{ cacheMode.description }}
                </p>
              </div>
            </div>
          </div>
        </fieldset>
      </template>
    </checkboxItem>
    <checkboxItem label="no_internet_layout" disableCheckbox>
      <template #description> There are multi layout of no Internet</template>
      <template #default>
        <fieldset>
          <legend class="sr-only">no internet layout</legend>
          <div class="space-y-5">
            <div
              v-for="noNet in noNetLayouts"
              :key="noNet.value"
              class="relative flex items-start"
            >
              <div class="flex items-center h-5">
                <input
                  v-model="appInfo.app_setting.no_internet_layout.type"
                  :id="noNet.title.toString()"
                  :aria-describedby="`${noNet.value}-description`"
                  name="noNet"
                  type="radio"
                  :value="noNet.value"
                  :checked="noNet.value === 1"
                  class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300"
                />
              </div>
              <div class="ml-3 text-sm">
                <label
                  :for="noNet.title.toString()"
                  class="font-medium text-gray-700 dark:text-slate-300"
                  >{{ noNet.title }}</label
                >
                <p :id="`${noNet.value}-description`" class="text-gray-500">
                  {{ noNet.description }}
                </p>
              </div>
            </div>
          </div>
        </fieldset>
        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 1"
          class="mt-2"
        >
          <fileSelect
            label="select your lottie file"
            :accept="['application/JSON']"
            v-model="appInfo.app_setting.no_internet_layout.lottieFile"
          ></fileSelect>
        </div>
        <div
          v-if="appInfo.app_setting.no_internet_layout.type === 2"
          class="mt-2"
        >
          <fileSelect
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
      :error="v$.app_setting.toolbar.$errors.map((e) => e.$message).join(',')"
    >
      <template #description> You can have different type of toolbar </template>
      <template #default>
        <fieldset>
          <legend class="sr-only">
            You can have different type of toolbar
          </legend>
          <div class="space-y-5">
            <div
              v-for="toolbarMode in toolbarsModes"
              :key="toolbarMode.value"
              class="relative flex items-start"
            >
              <div class="flex items-center h-5">
                <input
                  v-model="appInfo.app_setting.toolbar.type"
                  :id="toolbarMode.title.toString()"
                  :aria-describedby="`${toolbarMode.value}-description`"
                  name="noNet"
                  type="radio"
                  :value="toolbarMode.value"
                  :checked="toolbarMode.value === 0"
                  class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300"
                />
              </div>
              <div class="ml-3 text-sm">
                <label
                  :for="toolbarMode.title.toString()"
                  class="font-medium text-gray-700 dark:text-slate-300"
                  >{{ toolbarMode.title }}</label
                >
                <p
                  :id="`${toolbarMode.value}-description`"
                  class="text-gray-500"
                >
                  {{ toolbarMode.description }}
                </p>
              </div>
            </div>
          </div>
        </fieldset>
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
      label="toolbar_custom_icon"
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
import { useNavigationStore } from "@/stores/navigation";
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid";
import useVuelidate from "@vuelidate/core";
import { requiredIf } from "@vuelidate/validators";
import { storeToRefs } from "pinia";

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

const { appInfo } = storeToRefs(useAppSettingStore());
const { steps, activeTabIndex } = storeToRefs(useNavigationStore());

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
  },
};

const v$ = useVuelidate(rules, appInfo);

const next = async () => {
  if (await v$.value.$validate()) {
    steps.value[2].status = "complete";
    steps.value[3].status = "current";
    activeTabIndex.value++;
  }
};
</script>

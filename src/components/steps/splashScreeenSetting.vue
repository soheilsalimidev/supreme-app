<template>
  <div class="flex flex-col">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <RadioGroup v-model="appInfo.app_setting.splash_screen.type">
        <RadioGroupLabel class="sr-only"> Server size </RadioGroupLabel>
        <div class="space-y-4">
          <RadioGroup v-model="appInfo.app_setting.splash_screen.type">
            <RadioGroupLabel
              class="text-base font-medium text-gray-900 dark:text-white"
            >
              Select a splash screen
            </RadioGroupLabel>

            <div
              class="mt-4 grid grid-cols-1 gap-y-6 sm:grid-cols-3 sm:gap-x-4"
            >
              <RadioGroupOption
                as="template"
                v-for="(splashList, index) in splashType"
                :key="index"
                :value="splashList.value"
                v-slot="{ checked, active }"
              >
                <div
                  :class="[
                    checked ? 'border-transparent' : 'border-gray-300',
                    active ? 'ring-2 ring-indigo-500' : '',
                    'relative bg-white border rounded-lg shadow-sm p-4 flex cursor-pointer focus:outline-none dark:bg-slate-700',
                  ]"
                >
                  <div class="flex-1 flex">
                    <div class="flex flex-col">
                      <RadioGroupLabel
                        as="span"
                        class="block text-sm font-medium text-gray-900 dark:text-white"
                      >
                        {{ splashList.title }}
                      </RadioGroupLabel>
                      <RadioGroupDescription
                        as="span"
                        class="mt-1 flex items-center text-sm text-gray-500 dark:text-slate-400"
                      >
                        {{ splashList.description }}
                      </RadioGroupDescription>
                    </div>
                  </div>
                  <CheckCircleIcon
                    :class="[
                      !checked ? 'invisible' : '',
                      'h-5 w-5 text-indigo-600',
                    ]"
                    aria-hidden="true"
                  />
                  <div
                    :class="[
                      active ? 'border' : 'border-2',
                      checked ? 'border-indigo-500' : 'border-transparent',
                      'absolute -inset-px rounded-lg pointer-events-none',
                    ]"
                    aria-hidden="true"
                  />
                </div>
              </RadioGroupOption>
            </div>
          </RadioGroup>
        </div>
      </RadioGroup>

      <div v-if="appInfo.app_setting.splash_screen.type === 2">
        <fileSelect
          label="image"
          file-name="splash_bg.jpg"
          v-model="appInfo.app_setting.splash_screen.image_path"
          :error="
            v$.app_setting.splash_screen.image_path.$errors
              .map((e) => e.$message)
              .join(',')
          "
        ></fileSelect>
      </div>
      <div v-else-if="appInfo.app_setting.splash_screen.type === 3">
        <color-picker
          v-model="appInfo.app_setting.splash_screen.splash_screen_g_c"
          :error="
            v$.app_setting.splash_screen.splash_screen_g_c.$errors
              .map((e) => e.$message)
              .join(',')
          "
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import useVuelidate from "@vuelidate/core";
import { helpers, required } from "@vuelidate/validators";
import { storeToRefs } from "pinia";
import CheckCircleIcon from "~icons/heroicons/check-circle";
const splashType = [
  { title: "Just icon and app name", description: "sajdkfhasdkjfh", value: 0 },
  {
    title: "Icon with animation",
    description: "asdjhaksjdhklajshdk",
    value: 1,
  },
  { title: "full screen image", description: "sdlkjfhaskldfhkasdjh", value: 2 },
  {
    title: "icon and app name with gradient color",
    description: "sdlkjfhaskldfhkasdjh",
    value: 3,
  },
];

const { appInfo } = storeToRefs(useAppSettingStore());

const rules = {
  app_setting: {
    splash_screen: {
      type: {
        required,
      },
      splash_screen_g_c: {
        required: helpers.withMessage(
          "enter valid package name",
          (value: string) => {
            if (appInfo.value.app_setting.splash_screen.type === 3)
              return value ? true : false;
            return true;
          },
        ),
      },
      image_path: {
        required: helpers.withMessage(
          "enter valid package name",
          (value: string) => {
            if (appInfo.value.app_setting.splash_screen.type === 2)
              return value ? true : false;
            return true;
          },
        ),
      },
    },
  },
};

const v$ = useVuelidate(rules, appInfo);
</script>

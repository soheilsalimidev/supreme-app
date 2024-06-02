<template>
  <div class="flex flex-col">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <RadioGroup v-model="appInfo.app_setting.splash_screen.type">
        <RadioGroupLabel class="sr-only">
          {{ $t("steps.splash_screeen_setting.server_size") }}
        </RadioGroupLabel>
        <div class="space-y-4">
          <RadioGroup v-model="appInfo.app_setting.splash_screen.type">
            <RadioGroupLabel
              class="text-base font-medium text-gray-900 dark:text-white"
            >
              {{ $t("steps.splash_screeen_setting.select_a_splash_screen") }}
            </RadioGroupLabel>

            <div
              class="mt-4 grid grid-cols-1 gap-y-6 sm:grid-cols-3 sm:gap-x-4"
            >
              <RadioGroupOption
                v-for="(splashList, index) in splashType"
                :key="index"
                v-slot="{ checked, active }"
                as="template"
                :value="splashList.value"
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
                  >
                    <div
                      :class="[
                        active ? 'border' : 'border-2',
                        checked ? 'border-indigo-500' : 'border-transparent',
                        'absolute -inset-px rounded-lg pointer-events-none',
                      ]"
                      aria-hidden="true"
                    ></div>
                  </CheckCircleIcon>
                </div>
              </RadioGroupOption>
            </div>
          </RadioGroup>

          <div v-if="appInfo.app_setting.splash_screen.type === 2">
            <fileSelect
              v-model="appInfo.app_setting.splash_screen.image_path"
              file-name="splash_bg.jpg"
              :error="
                v$.app_setting.splash_screen.image_path.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
              :label="$t('steps.splash_screeen_setting.image')"
            >
            </fileSelect>
          </div>
          <div v-else-if="appInfo.app_setting.splash_screen.type === 3">
            <color-picker
              v-model="appInfo.app_setting.splash_screen.splash_screen_g_c"
              :error="
                v$.app_setting.splash_screen.splash_screen_g_c.$errors
                  .map((e) => e.$message)
                  .join(',')
              "
              :label="$t('steps.splash_screeen_setting.select_your_color')"
            >
            </color-picker>
          </div>
        </div>
      </RadioGroup>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import { useI18nValidators } from "@/utils/i18n-validators";
import useVuelidate from "@vuelidate/core";
import { storeToRefs } from "pinia";
import { useI18n } from "vue-i18n";
import CheckCircleIcon from "~icons/heroicons/check-circle";

const { t } = useI18n();
const splashType = [
  { title: t("splashType.justIconAndAppName"), description: "", value: 0 },
  {
    title: t("splashType.IconWithAnimation"),
    description: "",
    value: 1,
  },
  { title: t("splashType.fullScreenImage"), description: "", value: 2 },
  {
    title: t("splashType.iconAndAppNameWithGradientColor"),
    description: "",
    value: 3,
  },
];

const { required, requiredIf } = useI18nValidators();
const { appInfo } = storeToRefs(useAppSettingStore());

const rules = {
  app_setting: {
    splash_screen: {
      type: {
        required,
      },
      splash_screen_g_c: {
        required: requiredIf(
          appInfo.value.app_setting.splash_screen.type === 3,
        ),
      },
      image_path: {
        required: requiredIf(
          appInfo.value.app_setting.splash_screen.type === 2,
        ),
      },
    },
  },
};

const v$ = useVuelidate(rules, appInfo);
</script>

<i18n lang="json">
{
  "en": {
    "splashType": {
      "justIconAndAppName": "Just icon and app name",
      "IconWithAnimation": "Icon with animation",
      "fullScreenImage": "full screen image",
      "iconAndAppNameWithGradientColor": "icon and app name with gradient color"
    }
  },
  "fa": {
    "splashType": {
      "justIconAndAppName": "فقط نماد و نام برنامه",
      "IconWithAnimation": "آیکون با انیمیشن",
      "fullScreenImage": "تصویر تمام صفحه",
      "iconAndAppNameWithGradientColor": "نماد و نام برنامه با رنگ گرادیان"
    }
  }
}
</i18n>

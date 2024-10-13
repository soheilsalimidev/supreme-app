<template>
  <div class="h-full overflow-hidden flex flex-col">
    <VToolbar></VToolbar>
    <header class="pb-24 bg-indigo-600">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <div class="py-5 flex items-center justify-center lg:justify-between">
          <div class="">
            <a href="#">
              <span class="sr-only">Logo</span>
              <img class="h-8 w-auto" src="/icon.svg" />
            </a>
          </div>
        </div>
      </div>
    </header>
    <main class="-mt-24">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <h1 class="sr-only">Create the app</h1>
        <div
          ref="grid"
          class="gap-4 items-start grid"
          :class="[!activeComponentFrame ? 'grid-cols-6' : 'grid-cols-3']"
        >
          <div
            class="grid gap-4"
            :class="[
              !activeComponentFrame
                ? 'lg:col-start-2 lg:col-span-4 sm:col-start-0 sm:col-span-6'
                : 'lg:col-span-2 sm:col-span-5',
            ]"
          >
            <section aria-labelledby="section-1-title" class="flex flex-col">
              <h2 id="section-1-title" class="sr-only">Steps</h2>
              <div class="rounded-lg bg-white dark:bg-slate-800 shadow grow">
                <div class="p-6">
                  <VSteps />
                </div>
              </div>
              <div ref="sectionOne"></div>
            </section>
          </div>

          <div class="gap-4 hidden lg:grid col-span-1">
            <section
              v-if="activeComponentFrame"
              aria-labelledby="section-2-title"
              class="h-96"
            >
              <h2 id="section-2-title" class="sr-only">Phone frame preview</h2>
              <div class="rounded-lg bg-transparent overflow-hidden">
                <div class="flex items-center justify-center">
                  <Transition
                    :enter-from-class="
                      locale === 'en'
                        ? 'translate-x-[100%] '
                        : '-translate-x-[100%]'
                    "
                    appear
                    enter-active-class="transition duration-1000"
                  >
                    <div
                      class="flex items-center justify-center flex-col h-full w-full"
                    >
                      <VFrame />
                      <Teleport
                        v-if="isSm"
                        :to="$refs.sectionOne as RendererElement"
                        :disabled="isXL && isSm"
                      >
                        <Transition
                          enter-active-class="animate__animated animate__bounceIn"
                          leave-active-class="animate__animated animate__bounceOut"
                        >
                          <div
                            v-if="previewWaring"
                            class="bg-yellow-50 border-s-4 border-yellow-400 p-4 m-4"
                          >
                            <div class="flex">
                              <div class="flex-shrink-0">
                                <ExclamationTriangleIcon
                                  class="h-5 w-5 text-yellow-400"
                                  aria-hidden="true"
                                >
                                </ExclamationTriangleIcon>
                              </div>
                              <div class="ml-3">
                                <p class="text-sm text-yellow-700">
                                  {{ $t("frame.be_aware_that_this_just_pr")
                                  }}<a
                                    href="#"
                                    class="font-medium underline text-yellow-700 hover:text-yellow-600"
                                  >
                                    {{ $t("frame.and_what_you_see_on_render") }}
                                  </a>
                                </p>
                              </div>
                            </div>
                          </div>
                        </Transition>
                      </Teleport>
                    </div>
                  </Transition>
                </div>
              </div>
            </section>
          </div>
        </div>
      </div>
    </main>
    <footer class="mt-auto mb-3">
      <div class="px-4 bg-gray-100 dark:bg-gray-900 font-display">
        <div
          class="text-sm text-gray-500 text-center sm:text-left dark:text-white"
        >
          <span class="flex gap-1 text-base"
            >{{ t("buildWith") }}
            <LineMdHeart :key="refreshHeart" class="text-red-500" />
            {{ t("by") }}
            <a
              target="_blank"
              href="https://soheilsalimidev.ir"
              class="border-b-2 border-indigo-500 border-dashed"
              >{{ t("soheil") }}
            </a>
          </span>
        </div>
      </div>
    </footer>
  </div>

  <VModal
    v-model="noJavaModal"
    color="error"
    :ok-text="t('okDown')"
    :cancel-text="t('ok')"
    :title="t('noJava')"
    :ok="() => {}"
    :cancel="() => {}"
  >
    <template #text>
      <h2 class="mb-2 text-lg font-semibold text-gray-900 dark:text-white">
        {{ t("noJavaFound") }}
      </h2>
      <p class="text-sm text-gray-500 dark:text-slate-300">
        The minimum version required is 17
      </p>
      <ul
        class="max-w-md space-y-1 text-gray-500 list-disc list-inside dark:text-gray-400"
      >
        <li>
          <a href="https://www.oracle.com/java/technologies/downloads/"
            >oracle</a
          >
        </li>
        <li>
          <a
            href="https://soft98.ir/software/692-sun-java-se-runtime-environment.html"
            >soft98</a
          >
        </li>

        <li>
          <a
            href="https://p30download.ir/fa/entry/74697/jdk-java-se-development-kit"
            >p30download</a
          >
        </li>
      </ul>
    </template>
    <template #icon>
      <LineMdAlertLoop />
    </template>
  </Vmodal>
  <NotificationGroup group="generic">
    <div
      class="fixed inset-0 flex items-end justify-start p-6 px-4 py-6 pointer-events-none"
    >
      <div class="w-full max-w-sm">
        <Notification
          v-slot="{ notifications }"
          enter="transform ease-out duration-300 transition"
          enter-from="translate-y-2 opacity-0 sm:translate-y-0 sm:translate-x-4"
          enter-to="translate-y-0 opacity-100 sm:translate-x-0"
          leave="transition ease-in duration-500"
          leave-from="opacity-100"
          leave-to="opacity-0"
          move="transition duration-500"
          move-delay="delay-300"
        >
          <div v-for="notification in notifications" :key="notification.id">
            <div
              v-if="notification.type === 'info'"
              class="flex w-full max-w-sm mx-auto mt-4 overflow-hidden bg-white rounded-lg shadow-md"
            >
              <div class="flex items-center justify-center w-12 bg-blue-500">
                <svg
                  class="w-6 h-6 text-white fill-current"
                  viewBox="0 0 40 40"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M20 3.33331C10.8 3.33331 3.33337 10.8 3.33337 20C3.33337 29.2 10.8 36.6666 20 36.6666C29.2 36.6666 36.6667 29.2 36.6667 20C36.6667 10.8 29.2 3.33331 20 3.33331ZM21.6667 28.3333H18.3334V25H21.6667V28.3333ZM21.6667 21.6666H18.3334V11.6666H21.6667V21.6666Z"
                  />
                </svg>
              </div>

              <div class="px-4 py-2 -mx-3">
                <div class="mx-3">
                  <span class="font-semibold text-blue-500">{{
                    notification.title
                  }}</span>
                  <p class="text-sm text-gray-600">
                    {{ notification.text }}
                  </p>
                </div>
              </div>
            </div>

            <div
              v-if="notification.type === 'warning'"
              class="flex w-full max-w-sm mx-auto mt-4 overflow-hidden bg-white rounded-lg shadow-md"
            >
              <div class="flex items-center justify-center w-12 bg-yellow-500">
                <svg
                  class="w-6 h-6 text-white fill-current"
                  viewBox="0 0 40 40"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M20 3.33331C10.8 3.33331 3.33337 10.8 3.33337 20C3.33337 29.2 10.8 36.6666 20 36.6666C29.2 36.6666 36.6667 29.2 36.6667 20C36.6667 10.8 29.2 3.33331 20 3.33331ZM21.6667 28.3333H18.3334V25H21.6667V28.3333ZM21.6667 21.6666H18.3334V11.6666H21.6667V21.6666Z"
                  />
                </svg>
              </div>

              <div class="px-4 py-2 -mx-3">
                <div class="mx-3">
                  <span class="font-semibold text-yellow-500">{{
                    notification.title
                  }}</span>
                  <p class="text-sm text-gray-600">
                    {{ notification.text }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </Notification>
      </div>
    </div>
  </NotificationGroup>
</template>

<script setup lang="ts">
import LineMdHeart from "~icons/line-md/heart";
import ExclamationTriangleIcon from "~icons/heroicons/exclamation-triangle";
import { Notification, NotificationGroup } from "notiwind";
import { useMagicKeys, whenever } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
import { storeToRefs } from "pinia";
import { RendererElement, ref } from "vue";
import { onMounted } from "vue";
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import { useNavigationStore } from "@/stores/navigation";
import { wrapGrid } from "animate-css-grid";
import { useI18n } from "vue-i18n";
import { breakpointsTailwind, useBreakpoints } from "@vueuse/core";
import { saveAppAsFile } from "./utils/save";

const breakpoints = useBreakpoints(breakpointsTailwind);
const isXL = breakpoints.greaterOrEqual("2xl");
const isSm = breakpoints.isGreater("lg");
const { t } = useI18n();
const refreshHeart = ref(1);
const { activeComponentFrame } = storeToRefs(useNavigationStore());
const noJavaModal = ref(false);
const keys = useMagicKeys();
const grid = ref<HTMLElement | null>();
const previewWaring = ref(true);

whenever(keys.Ctrl_s, () => saveAppAsFile());

setInterval(() => {
  refreshHeart.value++;
}, 10000);
setInterval(() => {
  previewWaring.value = !previewWaring.value;
}, 360000);

onMounted(async () => {
  wrapGrid(grid.value!, { duration: 600 });
  try {
    if (await invoke<string>("check_java")) return;
  } catch (error) {
    console.error(error);
  }
  noJavaModal.value = true;
});
const { locale } = useI18n({ useScope: "global" });
</script>

<i18n>
{
  "en": {
    "buildWith": "build with",
    "by": "by",
    "soheil": "soheil salimi",
  "noJavaFound":"No Java JDK found, You can download it from the any  of following links",
  "noJava":"No Java JDK found",
  "okDown":"ok, i download" , "ok":"ok",
  "lang":"lang"
  },
  "fa": {
  "buildWith": "ساخته شده با",
  "by": "توسط",
    "soheil": "سهیل سلیمی",
  "noJavaFound":"try to download from the following sources:",
  "noJava":"No Java JDK found",
  "okDown":"ok, i download" , "ok":"ok",  "lang":"زبان"
  }
}
</i18n>

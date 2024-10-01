<template>
  <div class="h-full overflow-hidden flex flex-col">
    <toolbar></toolbar>
    <header class="pb-24 bg-indigo-600">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <div class="py-5 flex items-center justify-center lg:justify-between">
          <div class="">
            <a href="#">
              <span class="sr-only">Logo</span>
              <img class="h-8 w-auto" src="/icon.svg" />
            </a>
          </div>

          <div class="ms-auto mx-4">
            <div class="inline-flex rounded-md shadow-sm rtl:flex-row-reverse">
              <button
                type="button"
                class="relative inline-flex items-center rounded-l-md border border-gray-300 bg-white px-4 py-2 text-sm font-bold text-gray-700 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500 font-display"
              >
                {{ t("lang") }}
              </button>
              <Menu as="div" class="relative -ml-px block w-full">
                <MenuButton
                  class="relative inline-flex items-center rounded-r-md border border-gray-300 bg-white px-2 py-2 text-sm font-medium text-gray-500 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                >
                  <span class="sr-only">Open langs</span>
                  <HeroiconsChevronDown16Solid
                    class="h-5 w-5"
                    aria-hidden="true"
                  />
                </MenuButton>
                <transition
                  enter-active-class="transition ease-out duration-100"
                  enter-from-class="transform opacity-0 scale-95"
                  enter-to-class="transform opacity-100 scale-100"
                  leave-active-class="transition ease-in duration-75"
                  leave-from-class="transform opacity-100 scale-100"
                  leave-to-class="transform opacity-0 scale-95"
                >
                  <MenuItems
                    class="absolute right-0 z-10 mt-2 -mr-1 w-32 origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                  >
                    <div class="py-1">
                      <MenuItem
                        v-for="(item, index) in $i18n.availableLocales"
                        :key="index"
                        v-slot="{ active }"
                      >
                        <a
                          :class="[
                            active
                              ? 'bg-gray-100 text-gray-900'
                              : 'text-gray-700',
                            'block px-4 py-2 text-sm',
                          ]"
                          @click="changeLocal(item)"
                          >{{ langs[item as keyof typeof langs] }}</a
                        >
                      </MenuItem>
                    </div>
                  </MenuItems>
                </transition>
              </Menu>
            </div>
          </div>

          <div
            class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in"
          >
            <input
              id="toggle"
              type="checkbox"
              name="toggle"
              :value="isDark"
              class="bg-yellow-300 border-yellow-500 mr-1 focus:ring-transparent toggle-checkbox absolute block w-6 h-6 rounded-full border-2 appearance-none cursor-pointer"
              @change="toggleDark()"
            />
            <label
              for="toggle"
              class="toggle-label block h-8 -ml-1 -mt-1 rounded-full bg-green-400 cursor-pointer"
            />
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
                  <steps />
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
                      <frame />
                      <Teleport
                        :to="$refs.sectionOne as RendererElement"
                        :disabled="isXL"
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

  <modal
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
  </modal>
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
import { useDark, useToggle } from "@vueuse/core";
import { Notification, NotificationGroup, notify } from "notiwind";
import { useMagicKeys, whenever } from "@vueuse/core";
import { invoke } from "@tauri-apps/api/core";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { RendererElement, ref, unref } from "vue";
import { save } from "@tauri-apps/plugin-dialog";
import { onMounted } from "vue";
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import HeroiconsChevronDown16Solid from "~icons/heroicons/chevron-down-16-solid";
import { useNavigationStore } from "@/stores/navigation";
import { wrapGrid } from "animate-css-grid";
import { useI18n } from "vue-i18n";
import { breakpointsTailwind, useBreakpoints } from "@vueuse/core";

const breakpoints = useBreakpoints(breakpointsTailwind);
const isXL = breakpoints.greaterOrEqual("2xl");
const { t } = useI18n();
const refreshHeart = ref(1);
const { appInfo, savePath } = storeToRefs(useAppSettingStore());
const { activeComponentFrame } = storeToRefs(useNavigationStore());
const isDark = useDark();
const toggleDark = useToggle(isDark);
const noJavaModal = ref(false);
const keys = useMagicKeys();
const langs = {
  fa: "فارسی",
  en: "english",
};
const grid = ref<HTMLElement | null>();
const previewWaring = ref(true);

whenever(keys.Ctrl_s, async () => {
  try {
    if (!savePath.value) {
      const selected = await save({
        filters: [
          {
            name: "iapp",
            extensions: ["iapp"],
          },
        ],
      });
      if (!selected) {
        return;
      }
      savePath.value = selected + ".iapp";
    }
    await invoke("save_config", {
      config: unref(appInfo),
      path: savePath.value,
    });
    notify(
      {
        group: "generic",
        title: "settings save",
        text: "your settings saved in " + savePath.value,
        type: "info",
      },
      3000,
    );
  } catch (error) {
    notify(
      {
        group: "generic",
        title: "settings saving failed",
        text: "error" + error,
        type: "warning",
      },
      3000,
    );
  }
});

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
  } catch (error) {}
  noJavaModal.value = true;
});
const { locale } = useI18n({ useScope: "global" });
const changeLocal = (item: string) => {
  locale.value = item;
  document.dir = item === "fa" ? "rtl" : "lrt";
  document.documentElement.lang = item;
};
</script>

<style scoped>
.toggle-checkbox:checked {
  right: 0;
  border: none;
  background-color: rgb(129 124 214);
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' className='h-10 w-10' fill='%23FFF' viewBox='0 0 24 24' stroke='rgb(129 124 214)' > <path strokeLinecap='round' strokeLinejoin='round' d='M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z' /> </svg>");
}
.toggle-checkbox:checked + .toggle-label {
  background-color: rgb(76 73 188);
}
</style>

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

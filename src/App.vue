<template>
  <div class="min-h-full overflow-hidden">
    <header class="pb-24 bg-indigo-600">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <div class="py-5 flex items-center justify-center lg:justify-between">
          <div class="">
            <a href="#">
              <span class="sr-only">Logo</span>
              <img class="h-8 w-auto" :src="icon" alt="Workflow" />
            </a>
          </div>

          <div class="ms-auto mx-4">
            <div class="inline-flex rounded-md shadow-sm rtl:flex-row-reverse">
              <button
                type="button"
                class="relative inline-flex items-center rounded-l-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              >
                {{ t("lang") }}
              </button>
              <Menu as="div" class="relative -ml-px block">
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
    <main class="-mt-24 pb-8">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <h1 class="sr-only">Page title</h1>
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
            <section aria-labelledby="section-1-title">
              <h2 id="section-1-title" class="sr-only">Steps</h2>
              <div class="rounded-lg bg-white dark:bg-slate-800 shadow">
                <div class="p-6">
                  <steps />
                </div>
              </div>
            </section>
          </div>

          <!-- Right column -->
          <div class="gap-4 hidden lg:grid col-span-1">
            <Transition
              :enter-active-class="
                locale !== 'fa'
                  ? 'animate__animated animate__fadeInRightBig'
                  : 'animate__animated animate__fadeInLeftBig'
              "
              :leave-active-class="
                locale !== 'fa'
                  ? 'animate__animated animate__fadeOutRight'
                  : 'animate__animated animate__fadeOutLeft'
              "
              :duration="{ enter: 1000, leave: 2000 }"
            >
              <section
                v-if="activeComponentFrame"
                aria-labelledby="section-2-title"
              >
                <h2 id="section-2-title" class="sr-only">
                  Phone frame preview
                </h2>
                <div class="rounded-lg bg-transparent overflow-hidden">
                  <div class="p-6 flex items-center justify-center">
                    <frame />
                  </div>
                </div>
              </section>
            </Transition>
          </div>
        </div>
      </div>
    </main>
    <footer>
      <div
        class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 lg:max-w-7xl bg-gray-100 dark:bg-gray-900"
      >
        <div
          class="text-sm text-gray-500 text-center sm:text-left dark:text-white"
        >
          <span class="flex gap-1"
            >{{ t("buildWith") }}
            <LineMdHeart :key="refreshHeart" class="text-red-500" />
            {{ t("by") }}
            <a
              href="https://github.com/soheilsalimidev/"
              class="hover:underline"
              >{{ t("soheil") }}</a
            >
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
    {{ t("noJavaFound") }}

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
import icon from "@/assets/vue.svg";
import { useDark, useToggle } from "@vueuse/core";
import { Notification, NotificationGroup, notify } from "notiwind";
import { useMagicKeys, whenever } from "@vueuse/core";
import { invoke } from "@tauri-apps/api";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { ref, unref } from "vue";
import { save } from "@tauri-apps/api/dialog";
import { onMounted } from "vue";
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import HeroiconsChevronDown16Solid from "~icons/heroicons/chevron-down-16-solid";
import { useNavigationStore } from "@/stores/navigation";
import { wrapGrid } from "animate-css-grid";
import { useI18n } from "vue-i18n";
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

onMounted(async () => {
  try {
    if (await invoke<string>("check_java")) return;
  } catch (error) {}
  noJavaModal.value = true;
  wrapGrid(grid.value!, { duration: 600 });
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

<i18n lang="josn">
{
  "en": {
    "buildWith": "build with",
    "by": "by",
    "soheil": "soheil salimi",
  "noJavaFound":"No Java found",
  "noJava":"No java",
  "okDown":"ok, i download" , "ok":"ok",
  "lang":"lang"
  },
  "fa": {
    "buildWith": "ساخته شده با",
    "by": "توسط",
    "soheil": "سهیل سلیمی",
    "noJavaFound":"جاوا پیدا نشد",
  "noJava":"No java",
  "okDown":"ok, i download" , "ok":"ok",  "lang":"زبان"
  }
}
</i18n>

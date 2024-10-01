<template>
  <div class="flex w-full">
    <div
      data-tauri-drag-region
      class="h-6 select-none flex justify-end w-full bg-indigo-700 px-2 gap-3 items-center"
    >
      <Menu as="div" class="relative inline-block me-auto" v-slot="{ open }">
        <div
          v-element-hover="[
            (handler) => (openMenu = handler),
            { delayLeave: 200 },
          ]"
        >
          <MenuButton
            class="inline-flex w-full justify-center rounded-md p-1 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
          >
            Files
          </MenuButton>
        </div>

        <transition
          enter-active-class="transition duration-100 ease-out"
          enter-from-class="transform scale-95 opacity-0"
          enter-to-class="transform scale-100 opacity-100"
          leave-active-class="transition duration-75 ease-in"
          leave-from-class="transform scale-100 opacity-100"
          leave-to-class="transform scale-95 opacity-0"
        >
          <MenuItems
            v-if="openMenu || openMenuHover || open"
            v-element-hover="[
              (handler) => (openMenuHover = handler),
              { delayLeave: 200 },
            ]"
            static
            class="absolute start-0 mt-2 w-36 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none z-50"
          >
            <div class="px-1 py-1">
              <MenuItem v-slot="{ active }" @click="saveAppAsFile">
                <button
                  :class="[
                    active ? 'bg-indigo-500 text-white' : 'text-gray-900',
                    'group flex w-full items-center rounded-md px-2 py-2 text-sm gap-2',
                  ]"
                >
                  <component
                    class="h-5 w-5 text-indigo-400"
                    :class="active && 'text-indigo-100'"
                    :is="
                      !active
                        ? SolarFileDownloadBoldDuotone
                        : SolarFileDownloadBroken
                    "
                    aria-hidden="true"
                  />
                  Save
                </button>
              </MenuItem>
              <MenuItem v-slot="{ active }" @click="importFromTheAppFile()">
                <button
                  :class="[
                    active ? 'bg-indigo-500 text-white' : 'text-gray-900',
                    'group flex w-full items-center rounded-md px-2 py-2 gap-2',
                  ]"
                >
                  <component
                    class="h-5 w-5 text-indigo-400"
                    :class="active && 'text-indigo-100'"
                    :is="
                      !active ? SolarFileLeftBoldDuotone : SolarFileLeftBroken
                    "
                    aria-hidden="true"
                  />
                  Import
                </button>
              </MenuItem>
            </div>
          </MenuItems>
        </transition>
      </Menu>

      <div class="flex gap-2">
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
          @click="appWindow.close()"
        >
          <SolarCloseCircleBroken
            class="text-red-100 p-[1px] hidden group-hover:block"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { getCurrentWindow } from "@tauri-apps/api/window";
import SolarCloseCircleBroken from "~icons/solar/close-circle-broken";
import SolarMaximizeBroken from "~icons/solar/maximize-broken";
import SolarMinusCircleBroken from "~icons/solar/minus-circle-broken";
import SolarFileDownloadBoldDuotone from "~icons/solar/file-download-bold-duotone";
import SolarFileLeftBoldDuotone from "~icons/solar/file-left-bold-duotone";
import SolarFileLeftBroken from "~icons/solar/file-left-broken";
import SolarFileDownloadBroken from "~icons/solar/file-download-broken";
import SolarMinimizeBroken from "~icons/solar/minimize-broken";
import { vElementHover } from "@vueuse/components";
import { ref } from "vue";
import { importFromTheAppFile, saveAppAsFile } from "@/utils/save";
const openMenuHover = ref(false);
const appWindow = getCurrentWindow();
const openMenu = ref(false);
const isMax = ref(false);
appWindow.listen("tauri://resize", async () => {
  isMax.value = await appWindow.isMaximized();
});
</script>

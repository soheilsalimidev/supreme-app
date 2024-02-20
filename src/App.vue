<template>
  <div class="min-h-full">
    <header class="pb-24 bg-indigo-600">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <div
          class="relative py-5 flex items-center justify-center lg:justify-between"
        >
          <div class="absolute left-0 flex-shrink-0 lg:static">
            <a href="#">
              <span class="sr-only">Logo</span>
              <img class="h-8 w-auto" :src="icon" alt="Workflow" />
            </a>
          </div>

          <div>
            <div
              class="relative inline-block w-12 mr-2 align-middle select-none transition duration-200 ease-in"
            >
              <input
                type="checkbox"
                name="toggle"
                id="toggle"
                @change="toggleDark()"
                :value="isDark"
                class="bg-yellow-300 border-yellow-500 mr-1 focus:ring-transparent toggle-checkbox absolute block w-6 h-6 rounded-full border-2 appearance-none cursor-pointer"
              />
              <label
                for="toggle"
                class="toggle-label block h-8 -ml-1 -mt-1 rounded-full bg-green-400 cursor-pointer"
              ></label>
            </div>
          </div>
        </div>
      </div>
    </header>
    <main class="-mt-24 pb-8">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <h1 class="sr-only">Page title</h1>
        <!-- Main 3 column grid -->
        <div class="grid grid-cols-1 gap-4 items-start lg:grid-cols-3 lg:gap-8">
          <!-- Left column -->
          <div class="grid grid-cols-1 gap-4 lg:col-span-2">
            <section aria-labelledby="section-1-title">
              <h2 class="sr-only" id="section-1-title">Section title</h2>
              <div
                class="rounded-lg bg-white dark:bg-slate-800 overflow-hidden shadow"
              >
                <div class="p-6">
                  <steps />
                </div>
              </div>
            </section>
          </div>

          <!-- Right column -->
          <div class="grid grid-cols-1 gap-4">
            <section aria-labelledby="section-2-title">
              <h2 class="sr-only" id="section-2-title">Section title</h2>
              <div class="rounded-lg bg-transparent overflow-hidden">
                <div class="p-6">
                  <frame />
                </div>
              </div>
            </section>
          </div>
        </div>
      </div>
    </main>
    <footer>
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 lg:max-w-7xl">
        <div
          class="border-t border-gray-200 py-8 text-sm text-gray-500 text-center sm:text-left"
        >
          <span class="flex gap-1"
            >&copy; build with <LineMdHeart class="text-red-500" /> by soheil
            salimi</span
          >
        </div>
      </div>
    </footer>
  </div>

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
                  <p class="text-sm text-gray-600">{{ notification.text }}</p>
                </div>
              </div>
            </div>

            <div
              class="flex w-full max-w-sm mx-auto mt-4 overflow-hidden bg-white rounded-lg shadow-md"
              v-if="notification.type === 'warning'"
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
                  <p class="text-sm text-gray-600">{{ notification.text }}</p>
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
import { Notification, NotificationGroup } from "notiwind";
const isDark = useDark();
const toggleDark = useToggle(isDark);
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

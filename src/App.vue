<template>
  <div class="min-h-full">
    <Popover as="header" class="pb-24 bg-indigo-600" v-slot="{ open }">
      <div class="max-w-3xl mx-auto px-4 sm:px-6 lg:max-w-7xl lg:px-8">
        <div
          class="relative py-5 flex items-center justify-center lg:justify-between"
        >
          <!-- Logo -->
          <div class="absolute left-0 flex-shrink-0 lg:static">
            <a href="#">
              <span class="sr-only">Logo</span>
              <img class="h-8 w-auto" :src="icon" alt="Workflow" />
            </a>
          </div>

          <div>
            <input type="checkbox" class="sr-only" id="darkmode-toggle" @change="toggleDark()" :value="isDark"/>
            <label for="darkmode-toggle" class="toggle">
              <span>Toggle dark mode</span>
            </label>
          </div>
        </div>
        <div
          class="hidden lg:block border-t border-white border-opacity-20 py-5"
        >
          <div class="grid grid-cols-3 gap-8 items-center">
            <div class="col-span-2">
              <nav class="flex space-x-4">
                <a
                  v-for="item in navigationStore.navigation"
                  :key="item.name"
                  :href="item.href"
                  :class="[
                    item.current ? 'text-white' : 'text-indigo-100',
                    'text-sm font-medium rounded-md bg-white dark:bg-slate-800 bg-opacity-0 px-3 py-2 hover:bg-opacity-10 dark:hover:bg-opacity-40',
                  ]"
                  :aria-current="item.current ? 'page' : undefined"
                >
                  {{ item.name }}
                </a>
              </nav>
            </div>
          </div>
        </div>
      </div>

      <TransitionRoot as="template" :show="open">
        <div class="lg:hidden">
          <TransitionChild
            as="template"
            enter="duration-150 ease-out"
            enter-from="opacity-0"
            enter-to="opacity-100"
            leave="duration-150 ease-in"
            leave-from="opacity-100"
            leave-to="opacity-0"
          >
            <PopoverOverlay class="z-20 fixed inset-0 bg-black bg-opacity-25" />
          </TransitionChild>

          <TransitionChild
            as="template"
            enter="duration-150 ease-out"
            enter-from="opacity-0 scale-95"
            enter-to="opacity-100 scale-100"
            leave="duration-150 ease-in"
            leave-from="opacity-100 scale-100"
            leave-to="opacity-0 scale-95"
          >
            <PopoverPanel
              focus
              class="z-30 absolute top-0 inset-x-0 max-w-3xl mx-auto w-full p-2 transition transform origin-top"
            >
              <div
                class="rounded-lg shadow-lg ring-1 ring-black ring-opacity-5 bg-white dark:bg-slate-800 divide-y divide-gray-200"
              >
                <div class="pt-3 pb-2">
                  <div class="flex items-center justify-between px-4">
                    <div>
                      <img
                        class="h-8 w-auto"
                        src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
                        alt="Workflow"
                      />
                    </div>
                    <div class="-mr-2">
                      <PopoverButton
                        class="bg-white dark:bg-slate-800 rounded-md p-2 inline-flex items-center justify-center text-gray-400 hover:text-gray-500 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500"
                      >
                        <span class="sr-only">Close menu</span>
                        <XMarkIcon class="h-6 w-6" aria-hidden="true" />
                      </PopoverButton>
                    </div>
                  </div>
                  <div class="mt-3 px-2 space-y-1">
                    <a
                      href="#"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >Home</a
                    >
                    <a
                      href="#"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >Profile</a
                    >
                    <a
                      href="#"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >Resources</a
                    >
                    <a
                      href="#"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >Company Directory</a
                    >
                    <a
                      href="#"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >Openings</a
                    >
                  </div>
                </div>
                <div class="pt-4 pb-2">
                  <div class="flex items-center px-5">
                    <div class="flex-shrink-0">
                      <img
                        class="h-10 w-10 rounded-full"
                        :src="user.imageUrl"
                        alt=""
                      />
                    </div>
                    <div class="ml-3 min-w-0 flex-1">
                      <div class="text-base font-medium text-gray-800 truncate">
                        {{ user.name }}
                      </div>
                      <div class="text-sm font-medium text-gray-500 truncate">
                        {{ user.email }}
                      </div>
                    </div>
                    <button
                      type="button"
                      class="ml-auto flex-shrink-0 bg-white dark:bg-slate-800 p-1 text-gray-400 rounded-full hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    >
                      <span class="sr-only">View notifications</span>
                      <BellIcon class="h-6 w-6" aria-hidden="true" />
                    </button>
                  </div>
                  <div class="mt-3 px-2 space-y-1">
                    <a
                      v-for="item in userNavigation"
                      :key="item.name"
                      :href="item.href"
                      class="block rounded-md px-3 py-2 text-base text-gray-900 font-medium hover:bg-gray-100 hover:text-gray-800"
                      >{{ item.name }}</a
                    >
                  </div>
                </div>
              </div>
            </PopoverPanel>
          </TransitionChild>
        </div>
      </TransitionRoot>
    </Popover>
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
              <div
                class="rounded-lg bg-white dark:bg-slate-800 overflow-hidden shadow"
              >
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
</template>

<script setup lang="ts">
import LineMdHeart from "~icons/line-md/heart";
import { BellIcon, XMarkIcon } from "@heroicons/vue/24/outline";
import icon from "@/assets/vue.svg";
import { useNavigationStore } from "@/stores/navigation";
import { useDark, useToggle } from '@vueuse/core'

const isDark = useDark()
const toggleDark = useToggle(isDark)
const navigationStore = useNavigationStore();
</script>

<style scoped>
.toggle {
  font-size: 1rem; /* 👈 change this to scale */
  border: 0.125em solid currentColor;
  border-radius: 2em;
  cursor: pointer;
  display: block;
  height: 2em;
  position: relative;
  width: 3.75em;
}
.toggle span {
  background-color: currentColor;
  border-radius: 2em;
  display: block;
  height: 1.5em;
  left: 0.25em;
  overflow: hidden;
  position: absolute;
  top: 0.16em;
  text-indent: -9999px;
  transition: left 0.25s;
  width: 1.5em;
  z-index: 2;
}

.toggle::before,
.toggle::after {
  content: "";
  display: block;
  border-radius: 1em;
  position: absolute;
  z-index: 1;
}

.toggle::after {
  box-shadow: 0.25em 0.25em #5901d8;
  height: 1.125em;
  right: 0.9em;
  top: 0.125em;
  width: 1.125em;
}

.toggle::before {
  background-color: #ffc409;
  height: 0.625em;
  outline: 0.25em dotted #ffc409;
  outline-offset: 0.125em;
  left: 0.7em;
  top: 0.7em;
  width: 0.625em;
}

input:checked ~ .toggle span {
  left: 1.9em;
}
</style>

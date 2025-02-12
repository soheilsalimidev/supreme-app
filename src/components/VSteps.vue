<template>
  <div class="flex flex-col items-center">
    <nav :aria-label="t('progress')" class="w-full" style="direction: rtl">
      <TransitionGroup tag="ol" role="list" class="flex items-center justify-around w-full px-14 flex-row-reverse"
        appear enter-active-class="animate__animated animate__fadeIn">
        <li v-for="(step, stepIdx) in steps" :key="stepIdx" class="flex justify-center items-center"
          :class="stepIdx !== 4 && 'grow'">
          <div v-if="stepIdx !== 4" class="w-full bg-gray-200 h-0.5 dark:bg-gray-700 transition"
            :class="step.status === 'complete' && 'dark:bg-indigo-500/90!'"></div>

          <div class="relative">
            <template v-if="step.status === 'complete'">
              <div class="absolute inset-0 flex items-center group" aria-hidden="true">
                <div class="h-0.5 w-full bg-indigo-600" />
              </div>
              <a
                class="relative w-8 h-8 flex items-center justify-center bg-indigo-600 rounded-full hover:bg-indigo-900">
                <CheckIcon class="w-5 h-5 text-white" aria-hidden="true" />
                <span class="sr-only">{{ step.name }}</span>
                <span
                  class="mt-20 bg-indigo-300/60 text-indigo-600 whitespace-nowrap p-1 rounded-md dark:text-indigo-100 dark:bg-indigo-500/90 font-bold hidden group-hover:block">{{
      $t(step.name) }}</span>
              </a>
            </template>
            <template v-else-if="step.status === 'current'">
              <div class="absolute inset-0 flex items-center" aria-hidden="true">
                <div class="h-0.5 w-full bg-gray-200 dark:bg-gray-700" />
              </div>
              <a class="relative w-8 h-8 flex items-center justify-center bg-white border-2 border-indigo-600 rounded-full dark:bg-gray-700"
                aria-current="step">
                <span class="h-2.5 w-2.5 bg-indigo-600 rounded-full dark:bg-gray-900 absolute" aria-hidden="true" />
                <span
                  class="mt-20 bg-indigo-300/60 text-indigo-600 whitespace-nowrap p-1 rounded-md dark:text-indigo-100 dark:bg-indigo-500/90 font-black font-display">{{
      $t(step.name) }}</span>
              </a>
            </template>
            <template v-else>
              <div class="absolute inset-0 flex items-center group" aria-hidden="true">
                <div class="h-0.5 w-full bg-gray-200 dark:bg-gray-700" />
              </div>
              <a
                class="group relative w-8 h-8 flex items-center justify-center bg-white border-2 border-gray-300 rounded-full hover:border-gray-400 dark:bg-gray-700 dark:border-gray-600">
                <span class="h-2.5 w-2.5 bg-transparent rounded-full group-hover:bg-gray-300" aria-hidden="true" />
                <span class="sr-only">{{ $t(step.name) }}</span>
                <span
                  class="mt-20 bg-indigo-300/60 text-indigo-600 whitespace-nowrap p-1 rounded-md dark:text-indigo-100 dark:bg-indigo-500/90 font-bold hidden group-hover:block">{{
      $t(step.name) }}</span>
              </a>
            </template>
          </div>
        </li>
      </TransitionGroup>
    </nav>

    <div class="w-full bg-gray-200 h-[1px] mt-14 dark:bg-gray-700" />

    <CustomScrollbar class="w-full p-4 overflow-y-scroll h-80 2xl:h-[36rem]" content-class="h-full w-full relative "
      wrapper-class="w-full h-full ">
      <Transition :enter-from-class="nextOrPrev ? 'translate-x-[100%] ' : '-translate-x-[100%]'
      " :leave-active-class="nextOrPrev
        ? '-translate-x-[100%] transition duration-1000'
        : 'translate-x-[100%] transition duration-1000'
      " appear enter-active-class="transition duration-1000">
        <component :is="activeComponent" :key="activeTabIndex" class="min-w-full absolute" />
      </Transition>
    </CustomScrollbar>

    <div class="w-full bg-gray-100 h-[1px] mt-1 dark:bg-gray-700" />
    <div class="flex w-full" style="direction: rtl">
      <div class="px-4 py-3 text-right sm:px-6 mt-auto me-auto">
        <button v-if="currentTab !== 4"
          class="inline-flex font-display justify-center py-2 px-4 border border-transparent shadow-xs text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          @click="next">
          {{ t("steps.next") }}
        </button>
      </div>

      <div class="px-4 py-3 text-right sm:px-6 mt-auto">
        <button v-if="currentTab !== 0"
          class="inline-flex font-display justify-center py-2 px-4 border border-transparent shadow-xs text-base font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          @click="previous">
          {{ t("steps.previous") }}
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import CheckIcon from "~icons/heroicons/check-solid";
import { storeToRefs } from "pinia";
import { useNavigationStore } from "@/stores/navigation";
import CustomScrollbar from "custom-vue-scrollbar";
import "custom-vue-scrollbar/dist/style.css";
import useVuelidate from "@vuelidate/core";
import { useI18n } from "vue-i18n";
const { t } = useI18n()
const { steps, activeComponent, activeTabIndex, currentTab, nextOrPrev } =
  storeToRefs(useNavigationStore());

const v$ = useVuelidate();

const previous = () => {
  nextOrPrev.value = false;
  steps.value.find((step) => step.status === "current")!.status = "upcoming";
  steps.value[--currentTab.value].status = "current";
  activeTabIndex.value++;
};

const next = async () => {
  if ((await v$.value.$validate()) || import.meta.env.DEV) {
    nextOrPrev.value = true;
    steps.value.find((step) => step.status === "current")!.status = "complete";
    steps.value[++currentTab.value].status = "current";
    activeTabIndex.value++;
  }
};
</script>

<style scoped>
.dark .scrollbar__thumb {
  background: gray;
  border-radius: 8px;
}
</style>

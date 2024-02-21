<template>
  <div class="flex flex-col items-center">
    <nav aria-label="Progress" class="w-fit">
      <XyzTransitionGroup
        role="list"
        class="flex items-center"
        appear
        tag="ol"
        duration="auto"
        xyz="fade flip-down origin-bottom stagger-1 delay-1"
      >
        <li
          v-for="(step, stepIdx) in steps"
          :key="step.name"
          :class="[
            stepIdx !== steps.length - 1 ? 'pr-8 sm:pr-20' : '',
            'relative',
          ]"
          @click="changeStep(step.id)"
        >
          <template v-if="step.status === 'complete'" class="group">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
              <div class="h-0.5 w-full bg-indigo-600" />
            </div>
            <a
              class="relative w-8 h-8 flex items-center justify-center bg-indigo-600 rounded-full hover:bg-indigo-900"
            >
              <CheckIcon class="w-5 h-5 text-white" aria-hidden="true" />
              <span class="sr-only">{{ step.name }}</span>
              <span
                class="mt-20 bg-indigo-500 whitespace-nowrap p-1 rounded-md dark:text-white hidden group-hover:block"
                >{{ step.name }}</span
              >
            </a>
          </template>
          <template
            v-else-if="step.status === 'current'"
            condition="step.status === 'current'"
          >
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
              <div class="h-0.5 w-full bg-gray-200 dark:bg-gray-700" />
            </div>
            <a
              class="relative w-8 h-8 flex items-center justify-center bg-white border-2 border-indigo-600 rounded-full dark:bg-gray-700"
              aria-current="step"
            >
              <span
                class="h-2.5 w-2.5 bg-indigo-600 rounded-full dark:bg-gray-900"
                aria-hidden="true"
              />
              <span
                class="mt-20 bg-indigo-500 whitespace-nowrap p-1 rounded-md dark:text-white"
                >{{ step.name }}</span
              >
            </a>
          </template>
          <template v-else class="group">
            <div class="absolute inset-0 flex items-center" aria-hidden="true">
              <div class="h-0.5 w-full bg-gray-200 dark:bg-gray-700" />
            </div>
            <a
              class="group relative w-8 h-8 flex items-center justify-center bg-white border-2 border-gray-300 rounded-full hover:border-gray-400 dark:bg-gray-700 dark:border-gray-600"
            >
              <span
                class="h-2.5 w-2.5 bg-transparent rounded-full group-hover:bg-gray-300"
                aria-hidden="true"
              />
              <span class="sr-only">{{ step.name }}</span>
              <span
                class="mt-20 bg-indigo-500 whitespace-nowrap p-1 rounded-md dark:text-white opacity-0 group-hover:opacity-100 transition-all"
                >{{ step.name }}</span
              >
            </a>
          </template>
        </li>
      </XyzTransitionGroup>
    </nav>

    <div
      class="mt-14 block w-full h-[31rem] p-4 overflow-scroll"
      xyz="fade appear-short-100% origin-top ease-in-out duration-7 delay-1.5"
      v-xyz="tabDirectionXyz"
    >
      <XyzTransition appear>
        <component :key="activeTabIndex" :is="activeComponent"></component>
      </XyzTransition>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { CheckIcon } from "@heroicons/vue/24/solid";
import { storeToRefs } from "pinia";
import { useNavigationStore } from "@/stores/navigation";
import { notify } from "notiwind";
import { ref } from "vue";

const tabDirectionXyz = ref("out-left-100% in-right-100%");

const { steps, activeComponent, activeTabIndex } =
  storeToRefs(useNavigationStore());

let currentTabId = 1;
const changeStep = (stepId: number) => {
  const goinogToStep = steps.value.find((step) => step.id === stepId)!;
  tabDirectionXyz.value =
    goinogToStep.id > currentTabId
      ? "out-left-100% in-right-100%"
      : "out-right-100% in-left-100%";
  if (
    steps.value.some(
      (step) => step.id < goinogToStep.id && step.status !== "complete",
    )
  ) {
    notify(
      {
        group: "generic",
        title: "Tasks not completed",
        text: "please complete all the steps before this before contiuning",
        type: "warning",
      },
      3000,
    );
  } else {
    steps.value.find((step) => step.status === "current")!.status = "upcoming";
    goinogToStep.status = "current";
    currentTabId = goinogToStep.id;
    activeTabIndex.value++;
  }
};
</script>

<style>
.slide-leave-active,
.slide-enter-active {
  transition: 1s;
}
.slide-enter {
  transform: translate(100%, 0);
}
.slide-leave-to {
  transform: translate(-100%, 0);
}

.slideback-leave-active,
.slideback-enter-active {
  transition: 1s;
}
.slideback-enter {
  transform: translate(-100%, 0);
}
.slideback-leave-to {
  transform: translate(100%, 0);
}
</style>

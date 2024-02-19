<template>
  <div class="flex flex-col items-center">
    <nav aria-label="Progress" class="w-fit">
      <ol role="list" class="flex items-center">
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
              <div class="h-0.5 w-full bg-gray-200" />
            </div>
            <a
              class="relative w-8 h-8 flex items-center justify-center bg-white border-2 border-indigo-600 rounded-full"
              aria-current="step"
            >
              <span
                class="h-2.5 w-2.5 bg-indigo-600 rounded-full"
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
              <div class="h-0.5 w-full bg-gray-200" />
            </div>
            <a
              class="group relative w-8 h-8 flex items-center justify-center bg-white border-2 border-gray-300 rounded-full hover:border-gray-400"
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
      </ol>
    </nav>
    <Transition name="fade" mode="out-in" class="h-[34rem] w-full mt-8 p-4">
      <component :is="activeComponent"></component>
    </Transition>
  </div>
</template>

<script lang="ts" setup>
import { CheckIcon } from "@heroicons/vue/24/solid";
import { storeToRefs } from "pinia";
import { useNavigationStore } from "@/stores/navigation";
import { notify } from "notiwind";
const { steps, activeComponent } = storeToRefs(useNavigationStore());

const changeStep = (stepId: number) => {
  const goinogToStep = steps.value.find((step) => step.id === stepId)!;
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
    steps.value.find((step) => (step.status = "current"))!.status = "complete";
    goinogToStep.status = "current";
  }
};
</script>

<template>
  <div
    class="z-40 w-full max-w-sm bg-white dark:bg-slate-800"
    :class="inputClass"
  >
    <div
      class="group relative flex rounded-md border border-gray-300 dark:border-slate-500 px-3 py-2 shadow-xs focus-within:border-indigo-600 focus-within:ring-1 focus-within:ring-indigo-600 text-opacity-90"
    >
      <label
        for="name"
        :class="labelClass"
        class="lrt:left-2 absolute -top-2 -mt-px inline-block bg-white dark:bg-slate-800 dark:bg-slate-800px-1 text-xs font-medium text-gray-900 dark:text-gray-100 rtl:right-2"
        >{{ label }}</label
      >
      <span
        class="mx-3 ml-auto block w-fit border-0 p-0 text-gray-900 dark:text-gray-100 placeholder-gray-500 focus:ring-0 sm:text-sm text-wrap"
        >{{ data }}</span
      >
      <div style="direction: ltr">
        <ColorPicker
          v-model:gradient-color="data"
          v-model:pure-color="data"
          lang="En"
          :use-type="gradient ? 'gradient' : 'pure'"
          :theme="isDark ? 'black' : 'white'"
        >
        </ColorPicker>
      </div>
    </div>
    <p v-if="error" class="mt-2 text-sm text-red-600">
      {{ error }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { ColorPicker } from "vue3-colorpicker";
import "vue3-colorpicker/style.css";
import { useDark } from "@vueuse/core";

const isDark = useDark();
withDefaults(
  defineProps<{
    label: string;
    modelValue: string;
    isTextColor?: string;
    labelClass?: string;
    inputClass?: string;
    error?: string;
    gradient?: boolean;
  }>(),
  {
    gradient: true,
    isTextColor: undefined,
    labelClass: undefined,
    inputClass: undefined,
    error: undefined,
  },
);

const data = defineModel<string>();
</script>

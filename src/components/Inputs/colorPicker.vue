<template>
  <div class="z-40 w-full max-w-sm bg-white dark:bg-slate-800">
    <div
      class="group relative flex rounded-md border border-gray-300 dark:border-slate-500 px-3 py-2 shadow-sm focus-within:border-indigo-600 focus-within:ring-1 focus-within:ring-indigo-600 text-opacity-90"
    >
      <label
        for="name"
        class="lrt:left-2 absolute -top-2 -mt-px inline-block bg-white dark:bg-slate-800 dark:bg-slate-800px-1 text-xs font-medium text-gray-900 dark:text-gray-100 rtl:right-2"
        >{{ label }}</label
      >
      <span
        class="mx-3 ml-auto block w-fit border-0 p-0 text-gray-900 dark:text-gray-100 placeholder-gray-500 focus:ring-0 sm:text-sm text-wrap"
        >{{ data }}</span
      >
      <div style="direction: ltr">
        <ColorPicker
          v-model:gradientColor="data"
          lang="En"
          use-type="gradient"
          :theme="isDark ? 'black' : 'white'"
        />
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
defineProps<{
  label: string;
  isTextColor?: string;
  modelValue: string;
  error?: string;
}>();

const data = defineModel<string>();
</script>

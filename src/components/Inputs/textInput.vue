<template>
  <div>
    <div
      class="relative border border-gray-300 rounded-md px-3 py-2 shadow-sm focus-within:ring-1 focus-within:ring-indigo-600 focus-within:border-indigo-600"
    >
      <label
        :for="label"
        class="absolute -top-2 left-2 -mt-px inline-block px-1 bg-white text-sm font-medium text-gray-900 dark:text-slate-400 dark:bg-slate-800"
        :class="labelClass"
        >{{ label }}</label
      >
      <input
        @blur="$emit('blur')"
        type="text"
        :name="label"
        :id="label"
        :disabled="disabled"
        :class="[
          error &&
            'border-red-300 text-red-900 placeholder-red-300 focus:outline-none focus:ring-red-500 focus:border-red-500 dark:placeholder-red-600',
          inputClass,
        ]"
        class="block w-full border-0 p-0 text-gray-900 placeholder-gray-500 focus:ring-0 dark:text-white dark:bg-slate-800 text-lg disabled:cursor-not-allowed disabled:border-gray-200 disabled:bg-gray-50 disabled:text-gray-500 dark:disabled:text-slate-500"
        v-model="modelValue"
        :placeholder="placeholder"
      />
      <div
        v-if="error"
        class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none"
      >
        <ExclamationCircleIcon
          class="h-5 w-5 text-red-500"
          aria-hidden="true"
        />
      </div>
    </div>
    <p class="mt-2 text-sm text-red-600" v-if="error">
      {{ error }}
    </p>
  </div>
</template>

<script setup lang="ts">
import ExclamationCircleIcon from "~icons/heroicons/exclamation-circle-solid";

defineProps<{
  label: string;
  placeholder: string;
  perfix?: string;
  disabled?: boolean;
  error?: string;
  labelClass?: string;
  inputClass?: string;
  value?: string;
}>();

defineEmits<{
  blur: [];
}>();

const modelValue = defineModel<string>({});
</script>

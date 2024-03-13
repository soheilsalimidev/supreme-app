<template>
  <fieldset>
    <legend class="sr-only">
      {{ label }}
    </legend>
    <div class="space-y-5">
      <div
        v-for="item in items"
        :key="item.value"
        class="relative flex items-start"
      >
        <div class="flex items-center h-5">
          <input
            :id="item.title.toString()"
            v-model="modelValue"
            :aria-describedby="`${item.value}-description`"
            :name="label"
            type="radio"
            :value="item.value"
            :checked="item.value === defaultValue"
            class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300"
          >
        </div>
        <div class="ml-3 text-sm">
          <label
            :for="item.title.toString()"
            class="font-medium text-gray-700 dark:text-slate-300"
          >{{ item.title }}</label>
          <p
            :id="`${item.value}-description`"
            class="text-gray-500"
          >
            {{ item.description }}
          </p>
        </div>
      </div>
    </div>
  </fieldset>
</template>

<script setup lang="ts">
defineProps<{
  items: {
    title: string;
    description: string;
    value: string | number;
  }[];
  label: string;
  error?: string;
  defaultValue: string | number;
}>();
const modelValue = defineModel();
</script>

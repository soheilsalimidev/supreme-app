<template>
  <div class="flex gap-2 justify-center items-center w-fit h-full">
    <span
      class="cursor-pointer block text-sm font-medium text-gray-700 dark:text-slate-300"
      @click="isActive = !isActive"
    >
      {{ label }}
    </span>
    <Switch
      v-model="isActive"
      :class="isActive ? 'bg-indigo-600' : 'bg-gray-200 dark:bg-gray-700'"
      class="relative inline-flex h-6 w-11 items-center rounded-full"
    >
      <span class="sr-only">{{ label }}</span>
      <span
        :class="
          isActive
            ? 'translate-x-6 rtl:-translate-x-6'
            : 'translate-x-1 rtl:-translate-x-1'
        "
        class="inline-block h-4 w-4 transform rounded-full bg-white transition"
      />
    </Switch>
  </div>
</template>

<script setup lang="ts">
defineProps<{ label: string }>();
const isActive = defineModel<boolean>({ required: true });
</script>

<template>
  <Disclosure
    v-slot="{ open }"
    as="div"
    @click="slots.default ? (modelValue = true) : (modelValue = !modelValue)"
  >
    <div
      class="focus:outline-none dark:bg-slate-700 hover:ring-2 hover:ring-indigo-500 max-w-full border rounded-lg shadow-sm p-2 cursor-pointer transition-all ease-in-out duration-200"
      :class="[open ? 'ring-2 ring-indigo-500 max-h-[30rem] mx-0' : 'max-h-20 mx-5']"
    >
      <DisclosureButton class="py-2" as="div">
        <div
          class="items-start relative flex"
          :class="[modelValue ? 'border-transparent' : 'border-gray-300']"
        >
          <div class="min-w-0 flex-1 text-sm">
            <label
              :for="label"
              class="font-medium text-gray-700 dark:text-white"
              >{{ label }}</label
            >
            <p :id="description" class="text-gray-500 dark:text-slate-400">
              <slot name="description" />
              {{ description }}
            </p>
          </div>
          <div class="ml-3 flex items-center h-5" v-if="!disableCheckbox">
            <input
              :id="label"
              :aria-describedby="description"
              :name="label"
              type="checkbox"
              class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded"
              v-model="modelValue"
              @click.stop=""
            />
          </div>
        </div>
        <p class="mt-2 text-sm text-red-600" v-if="error">
          {{ error }}
        </p>
      </DisclosureButton>
      <transition
        enter-active-class="transition duration-100 ease-out delay-100"
        enter-from-class="transform scale-95 opacity-0"
        enter-to-class="transform scale-100 opacity-100"
        leave-active-class="transition duration-75 ease-out"
        leave-from-class="transform scale-100 opacity-100"
        leave-to-class="transform scale-95 opacity-0"
      >
        <DisclosurePanel>
          <slot />
        </DisclosurePanel>
      </transition>
    </div>
  </Disclosure>
</template>

<script setup lang="ts">
import { useSlots } from "vue";

const slots = useSlots();
const modelValue = defineModel<boolean | undefined>();
withDefaults(
  defineProps<{
    label: string;
    description?: string;
    error?: string;
    disableCheckbox?: boolean;
  }>(),
  { disableCheckbox: false },
);
</script>

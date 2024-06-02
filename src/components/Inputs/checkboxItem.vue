<template>
  <Disclosure
    as="div"
    @click.self="
      slots.default ? (modelValue = true) : (modelValue = !modelValue)
    "
  >
    <div
      class="focus:outline-none dark:bg-slate-700 hover:ring-2 hover:ring-indigo-500 max-w-full border rounded-lg shadow-sm p-2 cursor-pointer transition-all ease-in-out duration-500"
      :class="[
        open ? 'ring-2 ring-indigo-500 max-h-[55rem]' : 'max-h-24 mx-5',
        open && (disableCheckbox || slots.default) ? 'mx-3' : 'mx-7',
      ]"
    >
      <DisclosureButton
        class="py-2"
        as="div"
        @click.self="slots.default ? (open = !open) : ''"
      >
        <div
          class="items-start relative flex"
          :class="[modelValue ? 'border-transparent' : 'border-gray-300']"
        >
          <div class="min-w-0 flex text-sm w-full ">
            <div
              class="w-fit grow"
              @click="slots.default ? (open = !open) : ''"
            >
              <label
                :for="label"
                class="font-bold text-gray-700 dark:text-slate-100 w-fit font-display text-base"
                >{{ label }}</label
              >
              <p
                :id="description"
                class="text-gray-500 dark:text-slate-400 w-fit"
              >
                <slot name="description">
                  {{ description }}
                </slot>
              </p>
            </div>
            <div >
              <slot name="end" />
            </div>
          </div>
          <div v-if="!disableCheckbox" class="ml-3 flex items-center h-5">
            <input
              :id="label"
              v-model="modelValue"
              :aria-describedby="description"
              :name="label"
              type="checkbox"
              class="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded"
              @click.stop=""
            />
          </div>
        </div>
        <p v-if="error && !open" class="mt-2 text-sm text-red-600">
          {{ error }}
        </p>
      </DisclosureButton>
      <transition
        enter-active-class="transition duration-200 ease-out delay-400"
        enter-from-class="transform scale-95 opacity-0"
        enter-to-class="transform scale-100 opacity-100"
        leave-active-class="transition duration-75 ease-out"
        leave-from-class="transform scale-100 opacity-100"
        leave-to-class="transform scale-95 opacity-0"
      >
        <DisclosurePanel v-if="open" class="p-4" static>
          <slot> </slot
        ></DisclosurePanel>
      </transition>
    </div>
  </Disclosure>
</template>

<script setup lang="ts">
import { useSlots } from "vue";

const slots = useSlots();
const modelValue = defineModel<boolean | undefined>();
const open = defineModel<boolean>("open", { default: false });

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

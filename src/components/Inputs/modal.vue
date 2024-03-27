<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <TransitionRoot as="template" :show="open">
    <Dialog as="div" class="relative z-10" @close="open = false">
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity">
          <div class="fixed inset-0 z-10 overflow-y-auto">
            <div
              class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
            >
              <TransitionChild
                as="template"
                enter="ease-out duration-300"
                enter-from="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
                enter-to="opacity-100 translate-y-0 sm:scale-100"
                leave="ease-in duration-200"
                leave-from="opacity-100 translate-y-0 sm:scale-100"
                leave-to="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              >
                <DialogPanel
                  class="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg dark:bg-slate-600"
                >
                  <div
                    class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4 dark:bg-slate-800"
                  >
                    <div class="sm:flex sm:items-start">
                      <div
                        class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full sm:mx-0 sm:h-10 sm:w-10"
                        :class="colors[color].icon"
                      >
                        <slot name="icon"> </slot>
                      </div>
                      <div
                        class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left w-full"
                      >
                        <DialogTitle
                          as="h3"
                          class="text-lg font-medium leading-6 text-gray-900 dark:text-white"
                        >
                          {{ title }}
                        </DialogTitle>
                        <div class="mt-2">
                          <p class="text-sm text-gray-500 dark:text-slate-400">
                            <slot> </slot>
                          </p>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div
                    class="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 dark:bg-slate-700"
                  >
                    <button
                      type="button"
                      :class="colors[color].button"
                      class="inline-flex w-full justify-center rounded-md border border-transparent px-4 py-2 text-base font-medium text-white shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 sm:ml-3 sm:w-auto sm:text-sm"
                      @click="
                        open = false;
                        ok();
                      "
                    >
                      {{ okText }}
                    </button>
                    <button
                      ref="cancelButtonRef"
                      type="button"
                      class="mt-3 inline-flex w-full justify-center rounded-md border border-gray-300 bg-white px-4 py-2 text-base font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
                      @click="
                        open = false;
                        cancel();
                      "
                    >
                      {{ cancelText }}
                    </button>
                  </div>
                </DialogPanel>
              </TransitionChild>
            </div>
          </div>
        </div></TransitionChild
      ></Dialog
    ></TransitionRoot
  >
</template>

<script setup lang="ts">
defineProps<{
  title: string;
  okText: string;
  cancelText: string;
  color: "warn" | "error" | "info";
  ok: () => void;
  cancel: () => void;
}>();

const colors = {
  warn: {
    icon: "bg-yellow-100 dark:bg-red-300",
    button: "bg-yellow-600 hover:bg-yellow-700  focus:ring-yellow-500 ",
  },
  info: {
    icon: "bg-indigo-100 dark:bg-indigo-300",
    button: "bg-indigo-600 hover:bg-indigo-700  focus:ring-indigo-500 ",
  },
  error: {
    icon: "bg-red-100 dark:bg-red-300",
    button: "bg-red-600 hover:bg-red-700  focus:ring-red-500 ",
  },
};
const open = defineModel<boolean>({ default: false });
</script>

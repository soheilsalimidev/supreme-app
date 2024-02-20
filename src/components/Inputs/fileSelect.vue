<template>
  <div class="grid grid-cols-1 space-y-2">
    <label class="text-sm font-bold text-gray-500 tracking-wide">{{
      label
    }}</label>
    <div class="flex items-center justify-center w-full" ref="dropZoneRef">
      <label
        v-if="!preview"
        class="flex flex-col rounded-lg border-4 border-dashed w-full h-60 p-10 group text-center"
      >
        <div
          class="h-full w-full text-center flex flex-col items-center justify-center"
        >
          <div v-if="isOverDropZone">
            <span class="text-sm text-gray-500">Drop your icon here</span>
          </div>
          <p class="pointer-none text-gray-500" v-else>
            <span class="text-sm">Drag and drop</span> files here <br />
            or
            <a
              class="text-indigo-600 hover:underline"
              @click="($refs.fileInput! as HTMLInputElement).click()"
              >select a file</a
            >
            from your computer
          </p>
        </div>
        <input
          type="file"
          class="hidden"
          ref="fileInput"
          accept=".jpg,.jpeg,.png"
          @change="
            onDrop(
              ($event.target as HTMLInputElement)!.files as unknown as File[],
            )
          "
        />
      </label>
      <div class="flex flex-wrap justify-start flex-col" v-else>
        <div class="w-6/12 sm:w-4/12 px-4">
          <img
            :src="preview"
            alt="logo"
            class="shadow rounded-full max-w-full h-auto align-middle border-none"
          />
        </div>
        <button
          class="mt-2 w-20 justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          @click="
            modelValue = undefined;
            preview = '';
          "
        >
          reset
        </button>
      </div>
    </div>

    <p class="mt-2 text-sm text-red-600" v-if="error">
      {{ error }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { useDropZone } from "@vueuse/core";
import { notify } from "notiwind";
import { ref } from "vue";

let props = withDefaults(
  defineProps<{
    label: string;
    error?: string;
    filterWarningText?: string;
    filterWarningTitle?: string;
    filterCondition?: (width: number, height: number) => boolean;
  }>(),
  { filterCondition: () => true },
);
const modelValue = defineModel();
const dropZoneRef = ref<HTMLDivElement>();
let preview = ref("");
const onDrop = async (files: File[] | null) => {
  if (files) {
    const { width, height, url } = await getImageDimensions(files[0]);
    if (props.filterCondition(width, height)) {
      modelValue.value = files;
      preview.value = url;
    } else {
      notify(
        {
          group: "generic",
          title: props.filterWarningTitle,
          text: props.filterWarningText,
          type: "warning",
        },
        5000,
      );
    }
  }
};

async function getImageDimensions(file: File) {
  let img = new Image();
  const url = URL.createObjectURL(file);
  img.src = url;
  await img.decode();
  let width = img.width;
  let height = img.height;
  return {
    width,
    height,
    url,
  };
}

const { isOverDropZone } = useDropZone(dropZoneRef, {
  onDrop,
  dataTypes: ["image/jpeg", "image/png"],
});
</script>

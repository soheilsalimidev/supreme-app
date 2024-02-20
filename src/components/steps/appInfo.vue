<template>
  <div class="flex flex-col">
    <div class="px-4 py-5 space-y-6 sm:p-6">
      <div class="grid grid-cols-3 gap-6">
        <textInput
          label="App name"
          placeholder="my amazing app"
          :error="v$.name.$errors.map((e) => e.$message).join(',')"
          @blur="v$.name.$touch"
          v-model="appInfo.name"
        />
        <textInput
          class="col-span-2"
          label="App packagename"
          placeholder="com.app.webapp"
          @blur="v$.package_name.$touch"
          :error="v$.package_name.$errors.map((e) => e.$message).join(',')"
          v-model="appInfo.package_name"
        />
      </div>
      <div class="grid grid-cols-1 space-y-2">
        <label class="text-sm font-bold text-gray-500 tracking-wide"
          >App icon</label
        >
        <div class="flex items-center justify-center w-full" ref="dropZoneRef">
          <label
            v-if="!get_logo"
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
                  ($event.target as HTMLInputElement)!
                    .files as unknown as File[],
                )
              "
            />
          </label>
          <div class="flex flex-wrap justify-start flex-col" v-else>
            <div class="w-6/12 sm:w-4/12 px-4">
              <img
                :src="get_logo"
                alt="logo"
                class="shadow rounded-full max-w-full h-auto align-middle border-none"
              />
            </div>
            <button
              class="mt-2 w-20 justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              @click="appInfo.icon_path = ''"
            >
              reset
            </button>
          </div>
        </div>

        <p class="mt-2 text-sm text-red-600" v-if="v$.icon_path.$error">
          {{ v$.icon_path.$errors.map((e) => e.$message).join(",") }}
        </p>
      </div>
    </div>

    <div class="px-4 py-3 text-right sm:px-6 mt-auto">
      <button
        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
        @click="next"
      >
        Next
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import useVuelidate from "@vuelidate/core";
import { required, helpers } from "@vuelidate/validators";
import { storeToRefs } from "pinia";
import { ref, watch } from "vue";
import { useDropZone } from "@vueuse/core";
import { notify } from "notiwind";
import { useNavigationStore } from "@/stores/navigation";

const { steps, activeTabIndex } = storeToRefs(useNavigationStore());
const dropZoneRef = ref<HTMLDivElement>();
const { appInfo, get_logo } = storeToRefs(useAppSettingStore());

const onDrop = async (files: File[] | null) => {
  if (files) {
    const { width, height } = await getImageDimensions(files[0]);
    if (
      (width === 512 && height === 512) ||
      (width === 48 && height === 48) ||
      (width === 192 && height === 192)
    ) {
      appInfo.value.icon_path = files;
    } else {
      notify(
        {
          group: "generic",
          title: "wrong image size",
          text: "your image size should be 512*512px or 192*192 or 48*48",
          type: "warning",
        },
        5000,
      );
    }
  }
};

const { isOverDropZone } = useDropZone(dropZoneRef, {
  onDrop,
  dataTypes: ["image/jpeg", "image/png"],
});

watch(
  () => appInfo.value.name,
  () =>
    (appInfo.value.package_name = appInfo.value.name
      .trim()
      .toLowerCase()
      .replaceAll("-", ".")
      .split(" ")
      .join(".")),
);

const rules = {
  package_name: {
    required,
    packname: helpers.withMessage(
      "enter valid package name",
      helpers.regex(/^[a-z][a-z0-9_]*(\.[a-z0-9_]+)*[a-z0-9_]*$/),
    ),
  },
  name: {
    required,
  },
  icon_path: {
    required,
  },
};

const v$ = useVuelidate(rules, appInfo);

async function getImageDimensions(file: File) {
  let img = new Image();
  img.src = URL.createObjectURL(file);
  await img.decode();
  let width = img.width;
  let height = img.height;
  return {
    width,
    height,
  };
}

const next = async () => {
  // if (await v$.value.$validate()) {
  steps.value[0].status = "complete";
  steps.value[1].status = "current";
  activeTabIndex.value++;
  // }
};
</script>

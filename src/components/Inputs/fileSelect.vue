<template>
  <div class="flex flex-col space-y-2">
    <label
      class="font-bold text-gray-500 tracking-wide font-display text-base h-fit"
      >{{ label }}</label
    >
    <div ref="dropZoneRef" class="flex items-center justify-center w-full grow">
      <label
        v-if="!preview"
        class="flex flex-col rounded-lg border-4 border-dashed w-full group text-center h-full"
      >
        <div
          class="h-full w-full text-center flex flex-col items-center justify-center"
        >
          <div v-if="isOverDropZone">
            <span class="text-sm text-gray-500">{{
              $t("inputs.file_select.drop_your_icon_here")
            }}</span>
          </div>
          <p v-else class="pointer-none text-gray-500 text-lg">
            <span class="text-lg">{{
              $t("inputs.file_select.drag_and_drop")
            }}</span>
            {{ $t("inputs.file_select.files_here") }} <br />
            {{ $t("inputs.file_select.or") }}
            <a
              class="text-indigo-600 hover:underline font-display font-bold"
              @click="selectFile"
              >{{ $t("inputs.file_select.select_a_file") }}</a
            >
            {{ $t("inputs.file_select.from_your_computer") }}
          </p>
          <p v-if="error" class="mt-2 text-sm text-red-600">
            {{ error }}
          </p>
        </div>
      </label>
      <div v-else class="flex flex-wrap justify-start flex-col">
        <div
          v-if="accept.some((type) => type.includes('png'))"
          class="w-6/12 sm:w-4/12 px-4"
        >
          <img
            :src="preview"
            class="shadow-sm rounded-full max-w-full h-auto align-middle border-none"
            :alt="$t('inputs.file_select.logo')"
          />
        </div>
        <span v-else class="text-start dark:text-slate-200">{{ preview }}</span>
        <button
          class="mt-2 min-w-20 justify-center py-2 px-4 border border-transparent shadow-xs text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-hidden focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
          @click="modelValue = undefined"
        >
          {{ $t("inputs.file_select.reset") }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useDropZone } from "@vueuse/core";
import { notify } from "notiwind";
import { ref, watch } from "vue";
import { listen } from "@tauri-apps/api/event";
import { convertFileSrc } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { computed } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
let props = withDefaults(
  defineProps<{
    label: string;
    fileName: string;
    error?: string;
    accept?: string[];
    filterWarningText?: string;
    filterWarningTitle?: string;
    filterCondition?: (width: number, height: number) => boolean;
  }>(),
  {
    filterCondition: () => true,
    accept: () => ["jpeg", "png", "jpg"],
    error: undefined,
    filterWarningText: undefined,
    filterWarningTitle: undefined,
  },
);

listen<string[]>("tauri://file-drop", (event) => onDrop(event.payload));

const modelValue = defineModel<string>();
const dropZoneRef = ref<HTMLDivElement>();

const preview = computed(() => {
  if (modelValue.value) {
    try {
      const fileUrl = convertFileSrc(modelValue.value);
      if (["png", "jpg", "jpeg"].includes(getUrlFileType(fileUrl) ?? "")) {
        return fileUrl;
      } else {
        return fileUrl.split("/").pop();
      }
    } catch (error) {
      console.error(error);
    }
    return undefined;
  } else return undefined;
});

const getUrlFileType = (url: string) => {
  const u = new URL(url);
  const ext = u.pathname.split(".").pop();
  return !ext || ext === "/" ? undefined : ext.toLowerCase();
};

const onDrop = async (files: string[] | null) => {
  if (files) {
    const fileUrl = convertFileSrc(files[0]);
    const ext = getUrlFileType(fileUrl);
    if (!ext || !props.accept.find((type) => type.includes(ext))) {
      notify(
        {
          group: "generic",
          title: t("wrong_format"),
          text: t("please_drop_currect_format"),
          type: "warning",
        },
        5000,
      );
      return;
    }

    if (props.accept.some((type) => type.includes("png"))) {
      const { width, height } = await getImageDimensions(fileUrl);
      if (!props.filterCondition(width, height)) {
        notify(
          {
            group: "generic",
            title: props.filterWarningTitle,
            text: props.filterWarningText,
            type: "warning",
          },
          5000,
        );
        return;
      }
    }
    modelValue.value = files[0];
  }
};

const { appInfo } = storeToRefs(useAppSettingStore());
watch(modelValue, () => {
  const path = appInfo.value.paths.find((path) => path.name === props.fileName);
  if (path) {
    path.path = modelValue.value ?? "";
  } else {
    appInfo.value.paths.push({
      name: props.fileName,
      path: modelValue.value ?? "",
    });
  }
});

const selectFile = async () => {
  const selected = (await open({
    multiple: false,
    filters: [
      {
        name: props.accept.join(","),
        extensions: props.accept,
      },
    ],
  })) as string;
  onDrop([selected]);
};

async function getImageDimensions(url: string) {
  let img = new Image();
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
  // onDrop,
  dataTypes: props.accept,
});
</script>

<i18n lang="json">
{
  "en": {
    "wrong_format": "wrong format",
    "please_drop_currect_format": "please drop currect format"
  },
  "fa": {
    "wrong_format": "فرمت اشتباه است",
    "please_drop_currect_format": "لطفا از فرمت jpg یا png استفاده کنید."
  }
}
</i18n>

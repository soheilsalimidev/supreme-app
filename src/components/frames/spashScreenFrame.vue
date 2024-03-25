<script setup lang="ts">
import { useAppSettingStore } from "@/stores/appSetting";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { storeToRefs } from "pinia";
import { computed } from "vue";

const { appInfo } = storeToRefs(useAppSettingStore());
const getLogo = computed(() => {
  try {
    return convertFileSrc(appInfo.value.icon_path)
  } catch (error) {
    console.error(error)
    return "" 
  }
});
</script>

<template>
  <Transition
    mode="out-in"
    enter-active-class="animate__animated animate__fadeIn"
    leave-active-class="animate__animated animate__fadeOut"
    appear
    class="bg-white h-full w-full overflow-hidden flex items-center justify-center flex-col"
  >
    <div v-if="appInfo.app_setting.splash_screen.type === 0">
      <img :src="getLogo" class="w-52 h-52" />
      <h2 class="mt-5 text-2xl">
        {{ appInfo.name }}
      </h2>
    </div>
    <div v-else-if="appInfo.app_setting.splash_screen.type === 1" class="">
      <div class="bounce">
        <img :src="getLogo" class="w-52 h-52" />
      </div>
      <h2 class="mt-5 text-2xl">
        {{ appInfo.name }}
      </h2>
    </div>
    <div v-else-if="appInfo.app_setting.splash_screen.type === 2">
      <img
        v-if="appInfo.app_setting.splash_screen.image_path"
        :src="convertFileSrc(appInfo.app_setting.splash_screen.image_path)"
        class="h-full w-full"
      />
      <p v-else>
        {{ $t("frames.spash_screen_frame.select_your_image") }}
      </p>
    </div>
    <div
      v-else-if="appInfo.app_setting.splash_screen.type === 3"
      :style="{
        backgroundImage: appInfo.app_setting.splash_screen.splash_screen_g_c,
      }"
    >
      <img :src="getLogo" class="w-52 h-52" />
      <h2 class="mt-5 text-2xl">
        {{ appInfo.name }}
      </h2>
    </div>
  </Transition>
</template>

<style>
.bounce {
  animation: bounce-in 3s infinite;
}
@keyframes bounce-in {
  0% {
    transform: scale(1);
  }
  40% {
    transform: scale(0.5);
  }
  80% {
    transform: scale(4);
  }
  100% {
    transform: scale(4);
  }
}
</style>

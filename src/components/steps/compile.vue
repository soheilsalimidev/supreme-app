<template>
  <div class="flex justify-center items-center h-full">
    <div
      :class="[
        state === 'running' &&
          'transition-all ease-in-out w-full h-full m-5 rounded-lg bg-white p-6 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] dark:bg-neutral-700 duration-700 ',
        state === 'start' && 'w-52 h-12 btn ',
      ]"
      class="text-white"
      @click="startRender"
    >
      <p
        v-if="state === 'start'"
        class="flex justify-center items-center h-full dark:text-white text:dark z-50"
      >
        Start
      </p>
      <div v-else>
        <div></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, unref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";

const { appInfo } = storeToRefs(useAppSettingStore());
const state = ref<"start" | "running" | "finished">("start");

const startRender = async () => {
  if (state.value === "start") {
    state.value = "running";
    try {
      console.log(JSON.stringify(unref(appInfo)));
      await invoke("render_app", { config: unref(appInfo) });
    } catch (error) {
      console.error(error);
    }
  }
};
</script>

<style>
.btn {
  margin: 0;
  padding: 0;
  border: none;
  outline: none;
  color: inherit;
  cursor: pointer;
  font-size: 16px;
  position: relative;
  border: 2px solid #000;
  animation: 0.5s collision ease-in;
  overflow: hidden;
  transition: 0.3s 0.1s;
}
.btn:active {
  transform: translateX(100px);
  background: rgba(255, 255, 255, 0.5);
}
.btn p {
  position: relative;
  color: #000;
  font-weight: 700;
  font-size: 1.2rem;
  z-index: 5;
  transition: 1s ease-in-out;
}
.btn:hover p {
  color: #fff;
}

.btn:before,
.btn:after {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background-color: #000;
}
.btn:hover:before {
  animation: collisionLeft 1s both;
  animation-direction: alternate;
}
.btn:hover:after {
  animation: collisionRight 1s both;
  animation-direction: alternate;
}
.btn:before {
  left: -30%;
}
.btn:after {
  left: 125%;
}
@keyframes collisionLeft {
  0% {
    left: -30%;
    width: 20px;
    height: 20px;
  }
  50% {
    left: 50%;
    width: 40px;
    height: 40px;
  }
  100% {
    left: 50%;
    width: 300px;
    height: 300px;
  }
}
@keyframes collisionRight {
  0% {
    left: 125%;
    width: 20px;
    height: 20px;
  }
  50% {
    left: 50%;
    width: 40px;
    height: 40px;
  }
  100% {
    left: 50%;
    width: 300px;
    height: 300px;
  }
}
</style>

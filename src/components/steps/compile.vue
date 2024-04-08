<template>
  <div>
    <div class="flex justify-center items-center h-full">
      <div
        ref="logsDiv"
        :class="[
          state === 'running' &&
            'w-full h-full m-5 rounded-lg bg-white p-6 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] dark:bg-neutral-700 duration-700 border-transparent',
          state === 'start' &&
            'w-52 h-12 btn text-inherit cursor-pointer text-base relative animate-[0.5s_collision_ease-in] overflow-hidden transition-[0.3s] duration-[0.1s] m-0 p-0 border-[none] border-2 border-solid border-indigo-300 dark:border-white',
        ]"
        class="text-white transition-all ease-in-out"
        @click="startRender"
      >
        <p
          v-if="state === 'start'"
          class="flex justify-center items-center h-full dark:text-white text-indigo-600 z-50 relative font-bold text-[1.2rem] transition-[1s] duration-[ease-in-out] hover:text-indigo-100 dark:hover:text-indigo-100"
        >
          {{ $t("steps.compile.start") }}
        </p>
        <div v-else>
          <TransitionGroup name="list" tag="ul">
            <li
              v-for="(log, index) in logs"
              :key="index"
              class="text-black dark:text-white text-left list-inside list-disc"
              style="direction: ltr"
            >
              {{ log }}
            </li>
          </TransitionGroup>
        </div>
      </div>
    </div>

    <modal
      v-model="complieFinish"
      :ok-text="t('save')"
      color="info"
      :cancel-text="t('cancel')"
      :title="t('finishedCom')"
      :ok="saveApk"
      :cancel="() => {}"
    >
      <div class="w-full flex flex-col">
        <h3>the code for asstes digital</h3>
        <div class="w-full">
          <div
            class="whitespace-pre-wrap bg-gray-100 p-2 m-2 dark:bg-gray-700 rounded-md"
            style="direction: ltr"
            v-html="assetes"
          ></div>
        </div>
      </div>
      <template #icon>
        <LineMdDownloadLoop />
      </template>
    </modal>
  </div>
</template>

<script setup lang="ts">
import { ref, unref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/api/dialog";
import { notify } from "notiwind";
import { useScroll } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import LineMdDownloadLoop from "~icons/line-md/download-loop";
import hljs from "highlight.js/lib/core";
import json from "highlight.js/lib/languages/json";
import "highlight.js/styles/github-dark.css";
// import LineMdClipboardPlus from "~icons/line-md/clipboard-plus";
hljs.registerLanguage("json", json);

const complieFinish = ref(false);
const logsDiv = ref<HTMLElement | null>(null);
const { y } = useScroll(logsDiv, { behavior: "smooth" });
const { t } = useI18n();
const logs = ref<string[]>(["start rendering"]);
const { appInfo } = storeToRefs(useAppSettingStore());
const state = ref<"start" | "running" | "finished">("start");

const assetes = ref("");

const saveApk = async () => {
  let selected = await save({
    filters: [
      {
        name: "app",
        extensions: ["apk"],
      },
    ],
  });
  if (!selected) {
    notify(
      {
        group: "generic",
        title: "saving app failed",
        text: "the app is in your desktop now",
        type: "warning",
      },
      3000,
    );
    selected = "desk";
    return;
  }

  await invoke("move_app", {
    path: selected + ".apk",
    config: unref(appInfo),
  });
};

const startRender = async () => {
  if (state.value === "start") {
    state.value = "running";
    try {
      await invoke("render_app", { config: unref(appInfo) });

      await listen<string>("render", (event) => {
        logs.value.push(event.payload);
        y.value = logsDiv.value!.scrollHeight;
      });

      await listen<string>("render_fineshed", (event) => {
        complieFinish.value = true;
        assetes.value = hljs.highlight(
          JSON.stringify(JSON.parse(event.payload), null, 4),
          {
            language: "json",
          },
        ).value;
      });
    } catch (error) {
      console.log(":asd")
      console.error(error);
    }
  }
};
</script>

<i18n>
  {
  "en":{
    "save":"save the app",
    "finishedCom":"finshed rendering"
  },
  "fa":{
    "save":"save the app",
    "finishedCom":"finshed rendering"
  }
}
</i18n>

<style scoped>
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
  background-color: #6366f1;
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

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>

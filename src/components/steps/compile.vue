<template>
  <div class="h-full">
    <div
      ref="progressDiv"
      class="flex justify-center items-center h-full rounded-md transition drop-shadow-lg"
      :class="[state === 'running' && 'progress-color']"
    >
      <div
        ref="logsDiv"
        :class="[
          state === 'running' &&
            'w-[98%] h-[98%] rounded-lg bg-white p-6 dark:bg-neutral-700 duration-700 border-transparent',
          state === 'start' &&
            'w-52 h-12 btn text-inherit cursor-pointer text-base relative animate-[0.5s_collision_ease-in] overflow-hidden transition-[0.3s] duration-[0.1s] m-0 p-0 border-[none] border-2 border-solid border-indigo-300 dark:border-white',
        ]"
        class="text-white transition-all ease-in-out"
        @click="startRender"
      >
        <p
          v-if="state === 'start'"
          class="flex justify-center items-center h-full dark:text-white text-indigo-600 z-50 relative font-bold text-[1.2rem] transition-[1s] duration-[ease-in-out] hover:text-indigo-100 dark:hover:text-indigo-100 font-display"
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
      v-model="isErr"
      :ok-text="t('ok')"
      color="error"
      :cancel-text="t('CopyLogFile')"
      :title="t('ErrorRendering')"
      :cancel="copyLog"
    >
      <div class="w-full flex flex-col">
        <h3 class="text-black">{{ t("errorRec") }}</h3>
        <span>{{ t("seeLogs") }}</span>
      </div>
      <template #icon>
        <LineMdAlertLoop />
      </template>
    </modal>

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
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/api/dialog";
import { notify } from "notiwind";
import { useScroll } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import LineMdDownloadLoop from "~icons/line-md/download-loop";
import anime from "animejs/lib/anime.es.js";
import hljs from "highlight.js/lib/core";
import json from "highlight.js/lib/languages/json";
import "highlight.js/styles/github-dark.css";
// import LineMdClipboardPlus from "~icons/line-md/clipboard-plus";
hljs.registerLanguage("json", json);

const complieFinish = ref(false);
const logsDiv = ref<HTMLElement | null>(null);
const progressDiv = ref<HTMLElement | null>(null);
const { y } = useScroll(logsDiv, { behavior: "smooth" });
const { t } = useI18n();
const logs = ref<string[]>(["start rendering"]);
const { appInfo } = storeToRefs(useAppSettingStore());
const state = ref<"start" | "running" | "finished">("start");
const isErr = ref(false);
const assetes = ref("");

let progressBarObject = {
  progressBarStartValue: 0,
  progressBarEndValue: 100,
  progressBarAnimationValue: 0 * 3.6, 
};

const updateProgressBar = (percentage: number, color = "#6366f1") => {
  let newValue = 0;
  if (progressBarObject.progressBarStartValue != 100) {
    newValue =
      Math.ceil(
        (progressBarObject.progressBarStartValue + percentage) / percentage,
      ) * percentage;
  }
  if (newValue > 100) {
    newValue = 100;
  }
  anime({
    targets: progressBarObject,
    progressBarStartValue: newValue,
    progressBarAnimationValue: newValue * 3.6,
    easing: "easeInOutExpo",
    round: 1,
    update: () => {
      progressDiv.value!.style.backgroundImage = `conic-gradient(
          ${color} ${progressBarObject.progressBarAnimationValue}deg,
          ${color} 5%,
          rgba(99, 102, 241, ${progressBarObject.progressBarAnimationValue}),
          transparent, 
          transparent ${progressBarObject.progressBarAnimationValue + 10}deg)`;
    },
    duration: 750,
  });
};

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
        title: t("saveFailed"),
        text: t("saveDesk"),
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

const copyLog = async () => {
  await invoke("copy_logs");
  notify(
    {
      group: "generic",
      title: t("LogsCopyed"),
      text: t("copyed"),
      type: "info",
    },
    3000,
  );
};

const startRender = async () => {
  if (state.value === "start") {
    state.value = "running";
    try {
      let steps = 1;
      const totalSteps = 10;
      console.log(appInfo.value)
      await invoke("render_app", { config: unref(appInfo) });
      setTimeout(()=>{
        updateProgressBar((steps * 100) / totalSteps);
      } , 750)

      await listen<string>("error", (_) => {
        updateProgressBar((steps * 100) / totalSteps, "#ef4444");
        logs.value.push("Failed to produce the apk");
        y.value = logsDiv.value!.scrollHeight;
        isErr.value = true;
      });

      await listen<string>("render", (event) => {
        steps = +1;
        updateProgressBar((steps * 100) / totalSteps);
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
      console.error(error);
    }
  }
};
</script>

<i18n>
  {
  "en":{
    "save":"save the app",
    "finishedCom":"finshed rendering",
    "seeLogs":"see the logs for more info",
    "ErrorRendering":"Error rendering",
    "LogsCopyed":"Logs copyed",
    "copyed":"The logs copyed to your clipboard",
    "errorRec":"we encorech an error",
    "saveFailed":"saving app failed" ,
    "saveDesk":"the app is in your desktop now"
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

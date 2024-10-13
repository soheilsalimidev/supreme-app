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

    <VModal
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
    </VModal>

    <VModal
      v-model="complieFinish"
      :ok-text="t('save')"
      color="info"
      :cancel-text="t('cancel')"
      :title="t('finishedCom')"
      :ok="saveApk"
      :cancel="() => {}"
    >
      <div class="w-full flex flex-col">
        <div class="mb-2 flex justify-between items-center">
          <p class="text-xl text-gray-900 dark:text-white">
            the code for assets digital
          </p>
        </div>
        <div class="relative rounded-lg">
          <div class="overflow-scroll max-h-full">
            <pre><code id="code-block" class="whitespace-pre" style="direction:ltr" v-html="assetes"></code></pre>
          </div>
          <div class="absolute top-2 right-2" v-if="isSupported">
            <button
              class="text-gray-900 dark:text-gray-400 m-0.5 hover:bg-gray-100 dark:bg-gray-800 dark:border-gray-600 dark:hover:bg-gray-700 rounded-lg py-2 px-2.5 inline-flex items-center justify-center bg-white border-gray-200 border"
            >
              <span
                @click="copy(source)"
                class="inline-flex items-center"
                v-if="!copied"
              >
                <svg
                  class="w-3 h-3 me-1.5"
                  aria-hidden="true"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="currentColor"
                  viewBox="0 0 18 20"
                >
                  <path
                    d="M16 1h-3.278A1.992 1.992 0 0 0 11 0H7a1.993 1.993 0 0 0-1.722 1H2a2 2 0 0 0-2 2v15a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V3a2 2 0 0 0-2-2Zm-3 14H5a1 1 0 0 1 0-2h8a1 1 0 0 1 0 2Zm0-4H5a1 1 0 0 1 0-2h8a1 1 0 1 1 0 2Zm0-5H5a1 1 0 0 1 0-2h2V2h4v2h2a1 1 0 1 1 0 2Z"
                  />
                </svg>
                <span class="text-xs font-semibold">Copy code</span>
              </span>
              <span
                id="success-message"
                class="inline-flex items-center"
                v-else
              >
                <svg
                  class="w-3 h-3 text-blue-700 dark:text-blue-500 me-1.5"
                  aria-hidden="true"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 16 12"
                >
                  <path
                    stroke="currentColor"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M1 5.917 5.724 10.5 15 1.5"
                  />
                </svg>
                <span
                  class="text-xs font-semibold text-blue-700 dark:text-blue-500"
                  >Copied</span
                >
              </span>
            </button>
          </div>
        </div>
      </div>
      <template #icon>
        <LineMdDownloadLoop />
      </template>
    </VModal>
  </div>
</template>

<script setup lang="ts">
import { ref, unref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import LineMdAlertLoop from "~icons/line-md/alert-loop";
import { useAppSettingStore } from "@/stores/appSetting";
import { storeToRefs } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { save } from "@tauri-apps/plugin-dialog";
import { notify } from "notiwind";
import { useScroll } from "@vueuse/core";
import { useI18n } from "vue-i18n";
import LineMdDownloadLoop from "~icons/line-md/download-loop";
import anime from "animejs/lib/anime.es.js";
// import LineMdClipboardPlus from "~icons/line-md/clipboard-plus";
import { highlighter } from "@/utils/shiki";
import { useClipboard } from "@vueuse/core";

const source = ref("");
const { copy, copied, isSupported } = useClipboard();

const complieFinish = ref(true);
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
      await invoke("render_app", { config: unref(appInfo) });
      setTimeout(() => {
        updateProgressBar((steps * 100) / totalSteps);
      }, 750);

      await listen<string>("error", () => {
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

      await listen<string>("render_fineshed", async (event) => {
        complieFinish.value = true;
        const jsonCode = JSON.stringify(JSON.parse(event.payload), null, 4);
        source.value = jsonCode;
        assetes.value = (await highlighter).codeToHtml(jsonCode, {
          lang: "json",
          theme: "tokyo-night",
        });
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

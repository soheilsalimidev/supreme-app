import { defineStore } from "pinia";
import { ref, computed } from "vue";
import appInfo from "@/components/steps/appInfo.vue";
import splashScreeenSetting from "@/components/steps/splashScreeenSetting.vue";
import webPageSetting from "@/components/steps/webPageSetting.vue";
import compile from "@/components/steps/compileApp.vue";
import introPage from "@/components/steps/introPage.vue";
import spashScreenFrame from "@/components/frames/spashScreenFrame.vue";
import introFrame from "@/components/frames/introFrame.vue";
import webPageFrame from "@/components/frames/webPageFrame.vue";

export const useNavigationStore = defineStore("navigation", () => {
  const nextOrPrev = ref(true); // true means next is clicked
  const componentsSteps = {
    appInfo,
    splashScreeenSetting,
    webPageSetting,
    compile,
    introPage,
  };

  const componentsFrame = {
    spashScreenFrame,
    webPageFrame,
    introFrame,
  };

  const activeTabIndex = ref(4);
  const currentTab = ref(0);

  const steps = ref<
    {
      name: string;
      id: number;
      status: "current" | "upcoming" | "complete";
      componentStep: keyof typeof componentsSteps;
      componentFrame?: keyof typeof componentsFrame;
    }[]
  >([
    {
      name: "stepsLabel.appInfo",
      id: 1,
      status: "current",
      componentStep: "appInfo",
    },
    {
      name: "stepsLabel.splashScreen",
      id: 2,
      status: "upcoming",
      componentFrame: "spashScreenFrame",
      componentStep: "splashScreeenSetting",
    },
    {
      name: "stepsLabel.webPageSettings",
      id: 3,
      status: "upcoming",
      componentFrame: "webPageFrame",
      componentStep: "webPageSetting",
    },
    {
      name: "stepsLabel.intro",
      id: 4,
      status: "upcoming",
      componentStep: "introPage",
      componentFrame: "introFrame",
    },
    {
      name: "stepsLabel.render",
      id: 5,
      status: "upcoming",
      componentStep: "compile",
    },
  ]);

  const activeComponentFrame = computed(() => {
    return steps.value[currentTab.value].componentFrame
      ? componentsFrame[steps.value[currentTab.value]!.componentFrame!]
      : undefined;
  });

  const activeComponent = computed(
    () => componentsSteps[steps.value[currentTab.value]!.componentStep],
  );

  return {
    steps,
    activeComponent,
    activeTabIndex,
    currentTab,
    activeComponentFrame,
    nextOrPrev,
  };
});

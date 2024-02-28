import { defineStore } from "pinia";
import { ref, computed } from "vue";
import appInfo from "@/components/steps/appInfo.vue";
import splashScreeenSetting from "@/components/steps/splashScreeenSetting.vue";
import webPageSetting from "@/components/steps/webPageSetting.vue";
import compile from "@/components/steps/compile.vue";
import introPage from "@/components/steps/introPage.vue";
import spashScreenFrame from "@/components/frames/spashScreenFrame.vue";
import webPageFrame from "@/components/frames/webPageFrame.vue";
export const useNavigationStore = defineStore("navigation", () => {
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
  };

  const activeTabIndex = ref(1);
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
      name: "app info",
      id: 1,
      status: "current",
      componentStep: "appInfo",
    },
    {
      name: "splash screen",
      id: 2,
      status: "upcoming",
      componentFrame: "spashScreenFrame",
      componentStep: "splashScreeenSetting",
    },
    {
      name: "web page settings",
      id: 3,
      status: "upcoming",
      componentFrame: "webPageFrame",
      componentStep: "webPageSetting",
    },
    {
      name: "Step 4",
      id: 4,
      status: "upcoming",
      componentStep: "introPage",
    },
    {
      name: "Step 5",
      id: 5,
      status: "upcoming",
      componentStep: "compile",
    },
  ]);

  const activeComponentFrame = computed(() => {
    return steps.value[currentTab.value].componentFrame
      ? //@ts-ignore
        componentsFrame[steps.value[currentTab.value]!.componentFrame]
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
  };
});

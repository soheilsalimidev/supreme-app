import { defineStore } from "pinia";
import { ref, computed, watch } from "vue";
import appInfo from "@/components/steps/appInfo.vue";
import splashScreeenSetting from "@/components/steps/splashScreeenSetting.vue";
import webPageSetting from "@/components/steps/webPageSetting.vue";
import compile from "@/components/steps/compile.vue";
import introPage from "@/components/steps/introPage.vue";
import spashScreenFrame from "@/components/frames/spashScreenFrame.vue";
import webPageFrame from "@/components/frames/webPageFrame.vue";
import { useI18n } from "vue-i18n";

export const useNavigationStore = defineStore("navigation", () => {
  const { t, locale } = useI18n({ useScope: "global" });
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
  >([]);

  watch(
    locale,
    () => {
      steps.value = [
        {
          name: t("stepsLabel.appInfo"),
          id: 1,
          status: "current",
          componentStep: "appInfo",
        },
        {
          name: t("stepsLabel.splashScreen"),
          id: 2,
          status: "upcoming",
          componentFrame: "spashScreenFrame",
          componentStep: "splashScreeenSetting",
        },
        {
          name: t("stepsLabel.webPageSettings"),
          id: 3,
          status: "upcoming",
          componentFrame: "webPageFrame",
          componentStep: "webPageSetting",
        },
        {
          name: t("stepsLabel.intro"),
          id: 4,
          status: "upcoming",
          componentStep: "introPage",
        },
        {
          name: t("stepsLabel.render"),
          id: 5,
          status: "upcoming",
          componentStep: "compile",
        },
      ];
    },
    { immediate: true },
  );

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

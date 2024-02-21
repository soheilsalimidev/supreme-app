import { defineStore } from "pinia";
import { ref, computed } from "vue";
import appInfo from "@/components/steps/appInfo.vue";
import splashScreeenSetting from "@/components/steps/splashScreeenSetting.vue";
import webPageSetting from "@/components/steps/webPageSetting.vue";
import compile from "@/components/steps/compile.vue";
import introPage from "@/components/steps/introPage.vue";

export const useNavigationStore = defineStore("navigation", () => {
  const components = {
    appInfo,
    splashScreeenSetting,
    webPageSetting,
    compile,
    introPage,
  };

  const activeTabIndex = ref(1);
  const steps = ref<
    {
      name: string;
      id: number;
      status: "current" | "upcoming" | "complete";
      component: keyof typeof components;
    }[]
  >([
    { name: "app info", id: 1, status: "complete", component: "appInfo" },
    {
      name: "splash screen",
      id: 2,
      status: "complete",
      component: "splashScreeenSetting",
    },
    {
      name: "web page settings",
      id: 3,
      status: "complete",
      component: "webPageSetting",
    },
    { name: "Step 4", id: 4, status: "complete", component: "introPage" },
    { name: "Step 5", id: 5, status: "current", component: "compile" },
  ]);

  const activeComponent = computed(
    () =>
      components[
        steps.value.find((step) => step.status === "current")!.component
      ],
  );

  return { steps, activeComponent, activeTabIndex };
});

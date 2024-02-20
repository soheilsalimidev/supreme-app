import { defineStore } from "pinia";
import { ref, computed } from "vue";
import appInfo from "@/components/steps/appInfo.vue";
import splashScreeenSetting from "@/components/steps/splashScreeenSetting.vue";

export const useNavigationStore = defineStore("navigation", () => {
  const components = {
    appInfo,
    splashScreeenSetting,
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
    { name: "splash screen", id: 2, status: "current", component: "splashScreeenSetting" },
    { name: "Step 3", id: 3, status: "upcoming", component: "appInfo" },
    { name: "Step 4", id: 4, status: "upcoming", component: "appInfo" },
    { name: "Step 5", id: 5, status: "upcoming", component: "appInfo" },
  ]);

  const activeComponent = computed(
    () =>
      components[
        steps.value.find((step) => step.status === "current")!.component
      ],
  );

  return { steps, activeComponent, activeTabIndex };
});

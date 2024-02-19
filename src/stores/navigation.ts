import { defineStore } from "pinia";
import { ref, computed } from "vue";
import appInfo from "@/components/steps/appInfo.vue";

export const useNavigationStore = defineStore("navigation", () => {
  const components = {
    appInfo,
  };

  const steps = ref<
    {
      name: string;
      id: number;
      status: "current" | "upcoming" | "complete";
      component: keyof typeof components;
    }[]
  >([
    { name: "app info", id: 1, status: "current", component: "appInfo" },
    { name: "Step 2", id: 2, status: "upcoming", component: "appInfo" },
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

  return { steps,activeComponent };
});

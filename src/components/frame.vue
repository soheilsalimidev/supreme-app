<template>
  <div class="flex flex-col">
    <div class="device device-google-pixel-6-pro ">
      <div class="device-frame ">
        <component
          :is="activeComponentFrame"
          :key="activeTabIndex"
          class="min-w-full"
        />
      </div>
      <div class="device-stripe" />
      <div class="device-header" />
      <div class="device-sensors" />
      <div class="device-btns" />
      <div class="device-power" />
      <div class="device-home" />
    </div>
    <Transition
      enter-active-class="animate__animated animate__bounceIn"
      leave-active-class="animate__animated animate__bounceOut"
    >
      <div
        v-if="previewWaring"
        class="bg-yellow-50 border-l-4 border-yellow-400 p-4 m-4"
      >
        <div class="flex">
          <div class="flex-shrink-0">
            <ExclamationTriangleIcon
              class="h-5 w-5 text-yellow-400"
              aria-hidden="true"
            />
          </div>
          <div class="ml-3">
            <p class="text-sm text-yellow-700">
              be aware that this just preview
              {{ " " }}
              <a
                href="#"
                class="font-medium underline text-yellow-700 hover:text-yellow-600"
              >
                and what you see on rendered app may be differnt
              </a>
            </p>
          </div>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { useNavigationStore } from "@/stores/navigation";
import { storeToRefs } from "pinia";
import { onMounted } from "vue";
import { ref } from "vue";
import ExclamationTriangleIcon from "~icons/heroicons/exclamation-triangle";

const { activeComponentFrame, activeTabIndex } = storeToRefs(
  useNavigationStore()
);

const previewWaring = ref(true);

onMounted(() => {
  setInterval(() => {
    previewWaring.value = !previewWaring.value;
  }, 360000);
});
</script>

<style scoped lang="scss">
.device {
  --devices-scale: 0.7;
}

.device,
.device::before,
.device::after,
.device *::before,
.device *::after {
  box-sizing: border-box;
  display: block;
}

.device {
  position: relative;
  transform: scale(1);
  z-index: 1;
}

.device .device-frame {
  z-index: 1;
}

.device .device-screen {
  background-color: #000;
  background-position: center center;
  background-size: cover;
  object-fit: cover;
  position: relative;
}

.device-google-pixel-6-pro {
  height: calc(var(--devices-scale) * 862px);
  width: calc(var(--devices-scale) * 404px);
}

.device-google-pixel-6-pro .device-frame {
  background: #121212;
  border-radius: calc(var(--devices-scale) * 28px);
  box-shadow: inset 0 0 calc(var(--devices-scale) * 12px) #8d8d86,
    inset 0 calc(var(--devices-scale) * 7px) 0 calc(var(--devices-scale) * 3px)
      #fdfdfc,
    inset 0 calc(var(--devices-scale) * -6px) 0 calc(var(--devices-scale) * 3px)
      #fdfdfc;
  height: calc(var(--devices-scale) * 862px);
  margin: 0 calc(var(--devices-scale) * 2px);
  padding: calc(var(--devices-scale) * 20px) calc(var(--devices-scale) * 12px)
    calc(var(--devices-scale) * 26px) calc(var(--devices-scale) * 12px);
  width: calc(var(--devices-scale) * 400px);
}

.device-google-pixel-6-pro .device-screen {
  border-radius: calc(var(--devices-scale) * 27px);
  height: calc(var(--devices-scale) * 816px);
  width: calc(var(--devices-scale) * 376px);
}

.device-google-pixel-6-pro .device-header {
  background: linear-gradient(to bottom, #8d8d86 0, #cbcbc8 30%, #cbcbc8 100%);
  height: calc(var(--devices-scale) * 10px);
  left: 50%;
  margin-left: calc(var(--devices-scale) * -147px);
  position: absolute;
  top: 0;
  width: calc(var(--devices-scale) * 294px);
}

.device-google-pixel-6-pro .device-stripe::after {
  background: linear-gradient(to top, #121212, #666661);
  border-radius: calc(var(--devices-scale) * 50px)
    calc(var(--devices-scale) * 50px) 0 0;
  bottom: 0;
  content: "";
  height: calc(var(--devices-scale) * 2px);
  left: 50%;
  margin-left: calc(var(--devices-scale) * -22px);
  position: absolute;
  width: calc(var(--devices-scale) * 44px);
}

.device-google-pixel-6-pro .device-stripe::before {
  background: #cbcbc8;
  bottom: 0;
  content: "";
  height: calc(var(--devices-scale) * 9px);
  left: 50%;
  margin-left: calc(var(--devices-scale) * 40px);
  position: absolute;
  width: calc(var(--devices-scale) * 11px);
}

.device-google-pixel-6-pro .device-sensors {
  background: #121212;
  border-radius: 50%;
  height: calc(var(--devices-scale) * 22px);
  left: 50%;
  margin-left: calc(var(--devices-scale) * -11px);
  margin-top: calc(var(--devices-scale) * -11px);
  position: absolute;
  top: calc(var(--devices-scale) * 39px);
  width: calc(var(--devices-scale) * 22px);
}

.device-google-pixel-6-pro .device-sensors::after,
.device-google-pixel-6-pro .device-sensors::before {
  content: "";
  position: absolute;
}

.device-google-pixel-6-pro .device-sensors::after {
  background: linear-gradient(to bottom, #121212, #666661);
  border-radius: 0 0 calc(var(--devices-scale) * 50px)
    calc(var(--devices-scale) * 50px);
  height: calc(var(--devices-scale) * 4px);
  left: 50%;
  margin-left: calc(var(--devices-scale) * -103px);
  top: calc(var(--devices-scale) * -18px);
  width: calc(var(--devices-scale) * 206px);
}

.device-google-pixel-6-pro .device-sensors::before {
  background: radial-gradient(
      farthest-corner at 20% 20%,
      #6074bf 0,
      transparent 40%
    ),
    radial-gradient(
      farthest-corner at 80% 80%,
      #513785 0,
      #24555e 20%,
      transparent 50%
    );
  border-radius: 50%;
  box-shadow: 0 0 calc(var(--devices-scale) * 1px)
    calc(var(--devices-scale) * 1px) rgba(255, 255, 255, 0.05);
  height: calc(var(--devices-scale) * 8px);
  left: calc(var(--devices-scale) * 7px);
  top: calc(var(--devices-scale) * 7px);
  width: calc(var(--devices-scale) * 8px);
}

.device-google-pixel-6-pro .device-btns {
  background: #b2b2ae;
  height: calc(var(--devices-scale) * 102px);
  position: absolute;
  right: 0;
  top: calc(var(--devices-scale) * 306px);
  width: calc(var(--devices-scale) * 3px);
}

.device-google-pixel-6-pro .device-power {
  background: #b2b2ae;
  height: calc(var(--devices-scale) * 58px);
  position: absolute;
  right: 0;
  top: calc(var(--devices-scale) * 194px);
  width: calc(var(--devices-scale) * 3px);
}

.device-google-pixel-6-pro.device-gold .device-frame {
  box-shadow: inset 0 0 calc(var(--devices-scale) * 12px) #ff962e,
    inset 0 calc(var(--devices-scale) * 7px) 0 calc(var(--devices-scale) * 3px)
      white,
    inset 0 calc(var(--devices-scale) * -6px) 0 calc(var(--devices-scale) * 3px)
      white;
}

.device-google-pixel-6-pro.device-gold .device-header {
  background: linear-gradient(to bottom, #ff962e 0, #ffd6ad 30%, #ffd6ad 100%);
}

.device-google-pixel-6-pro.device-gold .device-stripe::after {
  background: linear-gradient(to top, #121212, #e07000);
}

.device-google-pixel-6-pro.device-gold .device-stripe::before {
  background: #ffd6ad;
}

.device-google-pixel-6-pro.device-gold .device-btns {
  background: #ffbd7a;
}

.device-google-pixel-6-pro.device-gold .device-power {
  background: #ffbd7a;
}

.device-google-pixel-6-pro.device-black .device-frame {
  box-shadow: inset 0 0 calc(var(--devices-scale) * 12px) black,
    inset 0 calc(var(--devices-scale) * 7px) 0 calc(var(--devices-scale) * 3px)
      #646668,
    inset 0 calc(var(--devices-scale) * -6px) 0 calc(var(--devices-scale) * 3px)
      #646668;
}

.device-google-pixel-6-pro.device-black .device-header {
  background: linear-gradient(to bottom, black 0, #323334 30%, #323334 100%);
}

.device-google-pixel-6-pro.device-black .device-stripe::after {
  background: linear-gradient(to top, #121212, black);
}

.device-google-pixel-6-pro.device-black .device-stripe::before {
  background: #323334;
}

.device-google-pixel-6-pro.device-black .device-btns {
  background: #191a1a;
}

.device-google-pixel-6-pro.device-black .device-power {
  background: #191a1a;
}
</style>

<template>
  <Menu as="div" class="relative inline-block" v-slot="{ open }">
    <div
      v-element-hover="[(handler) => (openMenu = handler), { delayLeave: 200 }]"
    >
      <MenuButton
        class="inline-flex w-full justify-center rounded-md p-1 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
      >
        {{ name }}
      </MenuButton>
    </div>

    <transition
      enter-active-class="transition duration-100 ease-out"
      enter-from-class="transform scale-95 opacity-0"
      enter-to-class="transform scale-100 opacity-100"
      leave-active-class="transition duration-75 ease-in"
      leave-from-class="transform scale-100 opacity-100"
      leave-to-class="transform scale-95 opacity-0"
    >
      <MenuItems
        v-if="openMenu || openMenuHover || open"
        v-element-hover="[
          (handler) => (openMenuHover = handler),
          { delayLeave: 200 },
        ]"
        static
        class="absolute start-0 mt-2 w-36 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none z-50"
      >
        <div class="px-1 py-1">
          <MenuItem
            v-for="(item, index) in menuItems"
            :key="index"
            v-slot="{ active }"
            @click="item.click"
          >
            <button
              :class="[
                active ? 'bg-indigo-500 text-white' : 'text-gray-900',
                'group flex w-full items-center rounded-md px-2 py-2 text-sm gap-2',
              ]"
            >
              {{ item.name }}
            </button>
          </MenuItem>
        </div>
      </MenuItems>
    </transition>
  </Menu>
</template>

<script lang="ts" setup>
import { vElementHover } from "@vueuse/components";
import { ref } from "vue";
const openMenuHover = ref(false);
const openMenu = ref(false);

defineProps<{
  menuItems: { name: string; click: () => void }[];
  name: string;
}>();
</script>

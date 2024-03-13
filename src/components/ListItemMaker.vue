<template>
  <div class="grid grid-cols-2 gap-4">
    <div
      v-for="(item, index) in modelValue.toSorted((item) =>
        item.Kind ? 0 : 1,
      )"
      :key="index"
    >
      <div v-if="item.Pair">
        <textInput
          v-model="item.Pair.first"
          label-class="dark:!bg-slate-700"
          input-class="dark:!bg-slate-700"
          :placeholder="$t('list_item_maker.open_the_website')"
          :label="$t('list_item_maker.name_of_item')"
        >
          <textInput
            v-model="item.Pair.second"
            label-class="dark:!bg-slate-700"
            input-class="dark:!bg-slate-700"
            :placeholder="$t('list_item_maker.mywebsite_com_hereigo')"
            :label="$t('list_item_maker.url_that_should_be_opened')"
          >
            <!-- TODO:ADD icon select -->
          </textInput></textInput
        >
      </div>
      <div v-else>
        <button
          :class="[
            'text-gray-900 dark:text-slate-100',
            'group flex w-full items-center rounded-md px-2 py-2 text-sm',
          ]"
        >
          <component
            :is="defaultItems.find((it) => it.value === item.Kind)!.icon"
            class="mr-2 h-5 w-5 text-violet-400"
            aria-hidden="true"
          >
          </component>
          {{ defaultItems.find((it) => it.value === item.Kind)!.name }}
        </button>
      </div>
    </div>
  </div>

  <Menu as="div" class="relative inline-block text-left z-50">
    <Float
      placement="bottom"
      portal
      enter="transition duration-200 ease-out"
      enter-from="scale-95 opacity-0"
      enter-to="scale-100 opacity-100"
      leave="transition duration-150 ease-in"
      leave-from="scale-100 opacity-100"
      leave-to="scale-95 opacity-0"
    >
      <div>
        <MenuButton
          class="inline-flex w-full justify-center rounded-md bg-black/20 px-4 py-2 text-sm font-medium text-white hover:bg-black/30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white/75"
        >
          {{ $t("list_item_maker.add") }}
          <ChevronDownIcon
            class="-mr-1 ml-2 h-5 w-5 text-violet-200 hover:text-violet-100"
            aria-hidden="true"
          >
          </ChevronDownIcon
          ></MenuButton>
      </div>

      <MenuItems>
        <div
          class="absolute left-1/2 mt-2 w-56 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black/5 focus:outline-none dark:bg-gray-700"
        >
          <div
            v-for="(item, index) in defaultItems.filter((item) => !item.added)"
            class="px-1 py-1"
            :key="index"
          >
            <MenuItem
              v-slot="{ active }"
              @click="
              item.name !== 'custom'
                ? (item.added = true) &amp;&amp; modelValue.push({ Kind: item.value })
                : modelValue.push({ Pair: { first: '', second: '' } })
              "
            >
            <button
              :class="[
                active
                  ? 'bg-violet-500 text-white'
                  : 'text-gray-900 dark:text-slate-100',
                'group flex w-full items-center rounded-md px-2 py-2 text-sm',
              ]"
            >
              <component
                :is="item.icon"
                :active="active"
                class="mr-2 h-5 w-5 text-violet-400"
                aria-hidden="true"
              >
              </component>
              {{ item.name }}
            </button>
            </MenuItem>
          </div>
        </div>
      </MenuItems>
    </Float>
  </Menu>
</template>

<script setup lang="ts">
import type { ItemMenu } from "@/stores/appSetting";
import ChevronDownIcon from "~icons/heroicons/chevron-down-solid";
import LineMdPlusCircle from "~icons/line-md/plus-circle";
import LineMdHomeMd from "~icons/line-md/home-md";
import LineMdPhone from "~icons/line-md/phone";
import LineMdStarAltFilled from "~icons/line-md/star-alt-filled";
import LineMdExternalLink from "~icons/line-md/external-link";
import LineMdExternalLinkRounded from "~icons/line-md/external-link-rounded";
import { useI18n } from "vue-i18n";
// import useVuelidate from "@vuelidate/core";
// import { helpers, required, url } from "@vuelidate/validators";
const { t } = useI18n()
const modelValue = defineModel<ItemMenu[]>({ required: true });

const defaultItems = [
  {
    name: "home_page",
    icon: LineMdHomeMd,
    added: false,
    value: 1,
  },
  {
    name: "about_us",
    icon: LineMdPhone,
    added: false,
    value: 2,
  },
  {
    name: "rate_us",
    icon: LineMdStarAltFilled,
    added: false,
    value: 3,
  },
  {
    name: "share_app",
    icon: LineMdExternalLink,
    added: false,
    value: 4,
  },
  {
    name: "exit",
    icon: LineMdExternalLinkRounded,
    added: false,
    value: 5,
  },
  {
    name: "custom",
    icon: LineMdPlusCircle,
    added: false,
  },
];

// const v$ = useVuelidate(
//   {
//     modelValue: {
//       $each: helpers.forEach({
//         Pair: {
//           first: { required, url },
//         },
//       }),
//     },
//   },
//   { modelValue },
// );
</script>


<i18n>

</i18n>

<template>
  <div class="grid grid-cols-2 gap-4">
    <div
      v-for="(item, index) in sortedDate"
      :key="index"
      :class="[item.Pair && 'col-span-2']"
    >
      <div v-if="item.Pair" class="flex flex-row gap-2">
        <textInput
          v-model="item.Pair.first"
          label-class="dark:!bg-slate-700"
          input-class="dark:!bg-slate-700"
          :error="
            v$.sortedDate.$each.$response.$errors[index].Pair.filter(
              ({ $validator }: any) => $validator === 'requiredFirs',
            )
              .map((e: any) => e.$message)
              .join(',')
          "
          :placeholder="$t('list_item_maker.open_the_website')"
          :label="$t('list_item_maker.name_of_item')"
        >
        </textInput>
        <textInput
          v-model="item.Pair.second"
          label-class="dark:!bg-slate-700"
          input-class="dark:!bg-slate-700"
          :error="
            v$.sortedDate.$each.$response.$errors[index].Pair.filter(
              ({ $validator }: any) =>
                $validator === 'requiredSec' || $validator === 'url',
            )
              .map((e: any) => e.$message)
              .join(',')
          "
          :placeholder="$t('list_item_maker.mywebsite_com_hereigo')"
          :label="$t('list_item_maker.url_that_should_be_opened')"
        >
          <!-- TODO:ADD icon select -->
        </textInput>
      </div>
      <div v-else>
        <button
          :class="[
            'text-gray-900 dark:text-slate-100',
            'group flex w-full items-center rounded-md px-2 py-2 text-sm',
          ]"
        >
          <component
            :is="defaultItemsIcons[defaultItems.findIndex((it) => it.value === item.Kind)]"
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
                  ? (item.added = true) && modelValue.push({ Kind: item.value })
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
                  :is="defaultItemsIcons[index]"
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
import useVuelidate from "@vuelidate/core";
import { helpers } from "@vuelidate/validators";
import { computed, ref } from "vue";
const { t } = useI18n({ useScope: "global" });

const modelValue = defineModel<ItemMenu[]>({ required: true });



const defaultItemsIcons = [
  LineMdHomeMd,
  LineMdPhone,
  LineMdStarAltFilled,
  LineMdExternalLink,
  LineMdExternalLinkRounded,
  LineMdPlusCircle
];

const defaultItems = ref([
  {
    name: t("items.home_page"),
    added: false,
    value: 1,
  },
  {
    name: t("items.about_us"),
    added: false,
    value: 2,
  },
  {
    name: t("items.rate_us"),
    added: false,
    value: 3,
  },
  {
    name: t("items.share_app"),
    added: false,
    value: 4,
  },
  {
    name: t("items.exit"),
    added: false,
    value: 5,
  },
  {
    name: t("items.custom"),
    added: false,
  },
]);

const sortedDate = computed(() => {
  return modelValue.value.toSorted((item) => (item.Kind ? 0 : 1));
});

function isUrl(urlStr: string) {
  try {
    new URL(urlStr);
    return true;
  } catch (e) {
    return false;
  }
}

const v$ = useVuelidate(
  {
    sortedDate: {
      $each: helpers.forEach({
        Pair: {
          requiredSec: helpers.withMessage(
            t("validations.required"),
            (value: any) => value.second,
          ),
          requiredFirs: helpers.withMessage(
            t("validations.required"),
            (value: any) => value.first,
          ),

          url: helpers.withMessage(t("validations.url"), (value: any) =>
            isUrl(value.second),
          ),
        },
      }),
    },
  },
  { sortedDate },
);
</script>

<i18n>

</i18n>

<template>
  <div class="w-full h-full flex justify-center items-center">
    <h2 class="font-bold dark:text-white text-xl">{{t('soon')}}</h2>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n()
</script>

<i18n>
{
  "en": {
    "soon": "Will be added soon"
  }
}
</i18n>

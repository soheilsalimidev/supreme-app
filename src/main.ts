import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import Notifications from "notiwind";
import { createI18n } from "vue-i18n";
import "animate.css";
import messages from '@intlify/unplugin-vue-i18n/messages'

const i18n = createI18n({
  legacy: false,
  locale: "fa",
  fallbackLocale: "en",
  messages,
});
const pinia = createPinia();

createApp(App).use(pinia).use(Notifications).use(i18n).mount("#app");

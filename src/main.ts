import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import Notifications from "notiwind";
import VueAnimXyz from '@animxyz/vue3'
import '@animxyz/core'

const pinia = createPinia();

createApp(App).use(pinia).use(Notifications).use(VueAnimXyz).mount("#app");

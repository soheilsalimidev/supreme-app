import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from "pinia";
import Notifications from "notiwind";

const pinia = createPinia();

createApp(App).use(pinia).use(Notifications).mount("#app");

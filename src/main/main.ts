import "@/assets/index.css";

import App from "./main.vue";
import { createApp } from "vue";
import router from "./router";

createApp(App).use(router).mount("#app");

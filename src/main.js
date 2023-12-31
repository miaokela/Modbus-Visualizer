import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from './router';
import store from './store';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css';

createApp(App).use(router).use(store).mount("#app");

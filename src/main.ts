import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import FocusTrap from "primevue/focustrap";
import ToastService from "primevue/toastservice";
import Ripple from 'primevue/ripple';
import Tooltip from "primevue/tooltip";
import router from "./router";

// 视图
import App from "./App.vue";

// 样式表
import "primeflex/primeflex.css";
import "primeicons/primeicons.css";

const app = createApp(App);
app.use(PrimeVue, { ripple: true });
app.use(router);
app.use(createPinia());
app.directive("focustrap", FocusTrap);
app.directive("tooltip", Tooltip);
app.directive('ripple', Ripple);
app.use(ToastService);

app.mount("#app");

import { createApp } from "vue";
import { createRouter, createWebHashHistory } from "vue-router";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";
import FocusTrap from "primevue/focustrap";
import ToastService from 'primevue/toastservice';

// 视图
import App from "./App.vue";
import PreLoad from "./components/PreLoad.vue";
import Main from "./components/Main.vue";
import Login from "./components/Login.vue";

// 样式表
import "primeflex/primeflex.css";
import "primeicons/primeicons.css";

localStorage.removeItem("isLoggedIn");
const routes = [
  { path: "/", component: PreLoad },
  { path: "/login", component: Login },
  { path: "/dashboard", component: Main },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
router.beforeEach((to, _from, next) => {
  const isLoggedIn = localStorage.getItem("isLoggedIn");
  if (to.path === "/login") {
    if (isLoggedIn) {
      next({ path: "/dashboard" });
    } else {
      next();
    }
  } else {
    if (!isLoggedIn) {
      if (to.path != "/") {
        next({ path: "/" });
      } else {
        next();
      }
    } else {
      next();
    }
  }
});

const app = createApp(App);
app.use(PrimeVue);
app.use(router);
app.use(createPinia())
app.directive("focustrap", FocusTrap)
app.use(ToastService);

app.mount("#app");

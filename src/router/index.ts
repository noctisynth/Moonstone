import { createRouter, createWebHashHistory } from "vue-router";
import PreLoad from "../components/PreLoad.vue";
import Login from "../components/Login.vue";
import Main from "../components/Main.vue";
import Register from "../components/Register.vue";
import { useLoginStore } from "../stores/login";

const routes = [
  { path: "/", component: PreLoad },
  { path: "/login", component: Login },
  { path: "/dashboard", component: Main },
  { path: "/register", component: Register },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
router.beforeEach((to, _from, next) => {
  const loginstore = useLoginStore();
  if (to.path === "/login") {
    if (loginstore.isLoggedIn) {
      next({ path: "/dashboard" });
    } else {
      next();
    }
  } else {
    if (!loginstore.isLoggedIn) {
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

export default router;

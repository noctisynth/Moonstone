import { createRouter, createWebHashHistory } from "vue-router";
import PreLoad from "../components/PreLoad.vue";
import Login from "../views/login.vue";
import Dashboard from "../views/dashboard.vue";
import Register from "../views/register.vue";
import { useLoginStore } from "../stores/login";

const routes = [
  { path: "/", component: PreLoad },
  { path: "/login", component: Login },
  { path: "/dashboard", component: Dashboard },
  { path: "/register", component: Register },
];
const router = createRouter({
  history: createWebHashHistory(),
  routes: routes,
});
router.beforeEach((to, _from, next) => {
  const loginstore = useLoginStore();
  if (to.path === "/login" || to.path === "/register") {
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

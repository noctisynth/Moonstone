import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { useSessionsStore } from "./sessions";

export const useLoginStore = defineStore("login", {
  state: () => {
    return {
      isLoggedIn: false,
      node: localStorage.getItem("node"),
      session_key: localStorage.getItem("session_key"),
      user_id: "",
      userProfile: {
        nickname: "",
        sex: false,
        country: "",
        favorites_icon: "",
      },
    };
  },
  actions: {
    async setNode(node: string) {
      this.node = node;
      localStorage.setItem("node", node);
    },
    async checkNode(node: string, store: boolean = true) {
      const res: { status: boolean; error: string } = JSON.parse(
        await invoke("check_node", { node: node })
      );
      if (res.status)
        if (store) {
          this.setNode(node);
        }
      return res;
    },
    async login(identity: string, password: string) {
      const sessionsStore = useSessionsStore();
      if (!this.node) {
        return { status: false, msg: "节点未设置！" };
      }
      let callback: { status: boolean; session_key: string; error: string } =
        JSON.parse(
          await invoke("login_handler", {
            server: this.node,
            identity: identity,
            password: password,
          })
        );
      if (callback["status"]) {
        this.isLoggedIn = true;
        this.session_key = callback.session_key;
        localStorage.setItem("session_key", callback.session_key);
        sessionsStore.init();
        return { status: true, msg: "登陆成功！" };
      } else {
        return {
          status: false,
          msg:
            "登录时出现错误: " +
            callback.error +
            "，请检查你的账号密码和网络连接。",
        };
      }
    },
    logout() {
      const sessionsStore = useSessionsStore();
      const node = this.node;
      localStorage.clear();
      this.$reset();
      this.node = node;
      sessionsStore.$reset();
    },
    async register(
      tuta_mail: string,
      username: string,
      nickname: string,
      password: string
    ) {
      let res: { status: boolean; error: string } = JSON.parse(
        await invoke("register_handler", {
          node: this.node,
          tutaMail: tuta_mail,
          username: username,
          nickname: nickname,
          password: password,
        })
      );
      if (res.status) {
        return { status: true, msg: "注册成功！" };
      } else {
        return { status: false, msg: "注册失败: " + res.error };
      }
    },
  },
});

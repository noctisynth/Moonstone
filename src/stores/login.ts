import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

export const useLoginStore = defineStore("login", {
  state: () => {
    return {
      isLoggedIn: false,
      node: localStorage.getItem("node"),
      session_key: localStorage.getItem("session_key"),
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
      this.isLoggedIn = false;
      this.session_key = null;
      localStorage.removeItem("session_key");
    },
  },
});

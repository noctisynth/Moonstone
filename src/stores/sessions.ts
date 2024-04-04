import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { useLoginStore } from "./login";

export const useSessionsStore = defineStore("sessions", {
  state: () => {
    return {
      tunnels: [],
      communities: JSON.parse(localStorage.getItem("communitues") || "[]"),
      database_initialized: false,
    };
  },

  actions: {
    async initDatabase() {
      if (!this.database_initialized) {
        const res: { status: boolean; error: string } = JSON.parse(
          await invoke("database_init", {})
        );
        if (res.status) {
          return { status: true, msg: "数据库初始化成功！" };
        } else {
          return { status: false, msg: "数据库初始化失败: " + res.error };
        }
      }
    },
    async newCommunity(
      name: string,
      security_level: number,
      cross_origin: boolean,
      token?: string
    ) {
      const loginstore = useLoginStore();
      if (!loginstore.node) return { status: false, msg: "节点未注册！" };
      const res: { status: boolean; error: string; id: number } = JSON.parse(
        await invoke("new_community_handler", {
          node: loginstore.node,
          sessionKey: loginstore.session_key,
          name: name,
          securityLevel: security_level,
          crossOrigin: cross_origin,
          token: token,
        })
      );
      if (res.status) {
        const sess = await this.newCommunitySession(
          loginstore.node,
          name,
          token
        );
        if (sess.status) return { status: true, msg: "社群创建成功！" };
        else return sess;
      } else {
        return { status: false, msg: "社群创建失败: " + res.error };
      }
    },
    async newCommunitySession(node: string, name: string, token?: string) {
      const res: { status: boolean; error: string } = JSON.parse(
        await invoke("session_new_community", {
          node: node,
          name: name,
          token: token,
        })
      );
      if (res.status) {
        return { status: true, msg: "社群创建成功！" };
      } else {
        return { status: false, msg: "数据库错误: " + res.error };
      }
    },
  },
});

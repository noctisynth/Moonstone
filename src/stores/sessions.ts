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
        return { status: true, msg: "社群创建成功！" };
      } else {
        return { status: false, msg: "社群创建失败: " + res.error };
      }
    },
  },
});

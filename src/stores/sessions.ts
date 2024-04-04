import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { useLoginStore } from "./login";
import Database from "@tauri-apps/plugin-sql";

export const useSessionsStore = defineStore("sessions", {
  state: () => {
    let db: any;
    return {
      db: db,
      tunnels: [],
      communities: JSON.parse(localStorage.getItem("communitues") || "[]"),
      initailized: false,
    };
  },
  actions: {
    async init() {
      if (this.initailized) return;
      const loginStore = useLoginStore();
      this.db = await Database.load("sqlite:sessions.db");
      const communities = await this.db.select(
        "SELECT * from community WHERE user = $1",
        [loginStore.session_key]
      );
      console.log(communities);
      this.initailized = true;
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
        return { status: true, msg: "社群创建成功！" };
      } else {
        return { status: false, msg: "社群创建失败: " + res.error };
      }
    },
  },
});

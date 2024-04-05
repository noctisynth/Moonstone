import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { useLoginStore } from "./login";
import Database from "@tauri-apps/plugin-sql";

export const useSessionsStore = defineStore("sessions", {
  state: () => {
    let db: Database = new Database("sqlite:sessions.db");
    let communities: any;
    let tunnels: any;
    return {
      db: db,
      tunnels: tunnels,
      communities: communities,
      initailized: false,
    };
  },
  actions: {
    async init() {
      if (this.initailized) return;
      const loginStore = useLoginStore();
      this.db = await Database.load("sqlite:sessions.db");
      this.communities = await this.db.select(
        "SELECT * from community WHERE user = $1",
        [loginStore.session_key]
      );
      this.initailized = true;
    },
    async newCommunity(
      name: string,
      security_level: number,
      cross_origin: boolean,
      token?: string
    ) {
      const loginStore = useLoginStore();
      if (!loginStore.node) return { status: false, msg: "节点未注册！" };
      const res: { status: boolean; error: string; id: number } = JSON.parse(
        await invoke("new_community_handler", {
          node: loginStore.node,
          sessionKey: loginStore.session_key,
          name: name,
          securityLevel: security_level,
          crossOrigin: cross_origin,
          token: token,
        })
      );
      if (res.status) {
        await this.db.execute(
          "INSERT OR REPLACE INTO community (user, node, sequence, name, token) VALUES ($1, $2, $3, $4, $5);  ",
          [loginStore.session_key, loginStore.node, res.id, name, token || null]
        );
        return { status: true, msg: "社群创建成功！" };
      } else {
        return { status: false, msg: "社群创建失败: " + res.error };
      }
    },
  },
});

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
        [loginStore.user_id]
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
      const res: { status: boolean; error: string; id: string } = await invoke(
        "new_community_handler",
        {
          node: loginStore.node,
          sessionKey: loginStore.session_key,
          name: name,
          securityLevel: security_level,
          crossOrigin: cross_origin,
          token: token,
        }
      );
      if (res.status) {
        await this.db.execute(
          "INSERT OR REPLACE INTO community (user, node, sequence, name, token) VALUES ($1, $2, $3, $4, $5);",
          [loginStore.user_id, loginStore.node, res.id, name, token || null]
        );
        this.communities = await this.db.select(
          "SELECT * from community WHERE user = $1",
          [loginStore.user_id]
        );
        return { status: true, msg: "社群创建成功！" };
      } else {
        return { status: false, msg: "社群创建失败: " + res.error };
      }
    },
    async addCommunity(community_id: string, token?: string) {
      const loginStore = useLoginStore();
      const res: { status: boolean; error: string } = await invoke(
        "join_community_handler",
        {
          node: loginStore.node,
          sessionKey: loginStore.session_key,
          userId: loginStore.user_id,
          communityId: community_id,
        }
      );
      if (res.status) {
        await this.db.execute(
          "INSERT OR REPLACE INTO community (user, node, sequence, name, token) VALUES ($1, $2, $3, $4, $5);  ",
          [
            loginStore.user_id,
            loginStore.node,
            community_id,
            "暂不支持",
            token || null,
          ]
        );
        this.communities = await this.db.select(
          "SELECT * from community WHERE user = $1",
          [loginStore.user_id]
        );
        return { status: true, msg: "社群加入成功！" };
      } else {
        return { status: false, msg: "社群加入失败: " + res.error };
      }
    },
  },
});

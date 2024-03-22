import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";

export const useLoginStore = defineStore("login", {
  state: () => {
    return {
      node: localStorage.getItem("node"),
    };
  },

  actions: {
    async checkNode(node: string, store: boolean = true) {
      const res: { status: boolean; error: string } = JSON.parse(
        await invoke("check_node", { node: node })
      );
      console.log(res)
      if (res.status)
        if (store) {
          this.node = node;
          localStorage.setItem("node", node);
        }
      return res;
    },
  },
});

import { defineStore } from "pinia";

export const useLoginStore = defineStore("login", {
  state: () => {
    return {
      node: localStorage.getItem("node"),
    };
  },

  actions: {
    checkNode(node: string, store: boolean = true) {
      if (store) {
        this.node = node;
        localStorage.setItem("node", node)
      }
    },
  },
});

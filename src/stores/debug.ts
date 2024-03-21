import { defineStore } from "pinia";

export const useDebugStore = defineStore("debug", {
  state: () => {
    return {
      debug: localStorage.getItem("debug") === "on",
    };
  },

  actions: {
    debugOn() {
      this.debug = true;
      localStorage.setItem("debug", "on");
    },
    debugOff() {
      this.debug = false;
      localStorage.setItem("debug", "off");
    },
  },
});

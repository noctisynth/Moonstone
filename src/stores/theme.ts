import { defineStore } from "pinia";

export function isLightTheme(): boolean {
  const colorSchemeValue = window
    .getComputedStyle(document.documentElement)
    .getPropertyValue("color-scheme");
  return colorSchemeValue === "light";
}

function isDarkMode(): boolean {
  if (window.matchMedia) {
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  }
  return false;
}

export const useThemeStore = defineStore("dark", {
  state: () => {
    let mode = localStorage.getItem("mode");
    if (mode === null) {
      if (isDarkMode()) {
        mode = "dark";
      } else {
        mode = "light";
      }
    }
    const is_dark = mode === "dark";
    let bg_class;
    if (is_dark) {
      bg_class = "bg-gray-900";
    } else {
      bg_class = "bg-primary-600";
    }
    return {
      dark: is_dark,
      bg_class: bg_class,
    };
  },

  actions: {
    setdark() {
      this.dark = true;
      localStorage.setItem("mode", "dark");
      this.bg_class = "bg-gray-900";
    },
    setlight() {
      this.dark = false;
      localStorage.setItem("mode", "light");
      this.bg_class = "bg-primary-600";
    },
    changeTheme(PrimeVue: any) {
      if (isLightTheme()) {
        PrimeVue.changeTheme(
          "aura-light-indigo",
          "aura-dark-indigo",
          "theme-link",
          () => {
            this.setdark();
          }
        );
      } else {
        PrimeVue.changeTheme(
          "aura-dark-indigo",
          "aura-light-indigo",
          "theme-link",
          () => {
            this.setlight();
          }
        );
      }
    },
  },
});

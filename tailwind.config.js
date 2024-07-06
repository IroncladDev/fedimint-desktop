/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: ["class"],
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        root: "var(--bg-root)",
        default: "var(--bg-default)",
        higher: "var(--bg-higher)",
        highest: "var(--bg-highest)",
        foreground: {
          DEFAULT: "var(--fg-default)",
          default: "var(--fg-default)",
          dimmer: "var(--fg-dimmer)",
          dimmest: "var(--fg-dimmest)",
        },
        outline: {
          DEFAULT: "var(--outline-default)",
          root: "var(--outline-root)",
          default: "var(--outline-default)",
          higher: "var(--outline-higher)",
          highest: "var(--outline-highest)",
        },
        accent: {
          DEFAULT: "var(--accent-default)",
          root: "var(--accent-root)",
          default: "var(--accent-default)",
          dimmer: "var(--accent-higher)",
          dimmest: "var(--accent-highest)",
        },
      },
    },
  },
  plugins: [],
};

import { defineConfig, defineGlobalStyles } from "@pandacss/dev";

const globalCss = defineGlobalStyles({
  'html, body': {
    '--header-height': '56px',
    fontFamily: 'Inter, sans-serif',
    bg: 'background',
    color: 'foreground',
    lineHeight: '1.5'
  }
})

export default defineConfig({
  preflight: true,
  outdir: "styled-system",
  globalCss,
  include: ["./src/**/*.{js,jsx,ts,tsx}"],
  exclude: [],
  hash: true,
  theme: {
    extend: {
      tokens: {
        colors: {
          background: { value: "#111111" },
          sidebar: { value: "#0d0d0d" },
          card: { value: "#262626" },
          primary: { value: "#2563eb" },
          "primary-hover": { value: "#1d4ed8" },
          "primary-light": { value: "#3b82f6" },
          muted: { value: "#a0a0a0" },
          foreground: { value: "#ffffff" },
          "foreground-muted": { value: "#e0e0e0" },
          surface: { value: "#171717" },
          border: { value: "#404040" },
          "surface-secondary": { value: "#1a1a1a" },
          transparent: { value: "rgba(0,0,0,0)" },
          black: { value: "#000000" },
        },
      },
    },
  },
});

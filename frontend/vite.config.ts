import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import { tanstackRouter } from "@tanstack/router-plugin/vite";

export default defineConfig({
  plugins: [
    tanstackRouter({ target: "solid", autoCodeSplitting: true }),
    solid(),
  ],
  server: {
    port: 3000,
    proxy: {
      "/api": "http://localhost:4000",
    },
  },
  build: {
    target: "esnext",
  },
});

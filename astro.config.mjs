import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

import pdf from "astro-pdf";

export default defineConfig({
  root: "./",
  base: "/",

  vite: {
    plugins: [tailwindcss()],
  },

  server: {
    headers: {
      "Cross-Origin-Opener-Policy": "same-origin",
    },
  },

  build: {
    concurrency: 4,
  },

  integrations: [pdf()],
});

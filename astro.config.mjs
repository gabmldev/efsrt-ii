import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

import pdf from "astro-pdf";

export default defineConfig({
  root: "./",
  base: "/",

  vite: {
    plugins: [tailwindcss()],
  },

  integrations: [pdf()],
});
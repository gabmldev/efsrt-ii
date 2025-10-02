import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "astro/config";

export default defineConfig({
  root: "./",
  base: "/",
  vite: {
    plugins: [tailwindcss()],
  },
});

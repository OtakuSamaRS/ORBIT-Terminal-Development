import { defineConfig } from "vite";

// The template serves static files from src/, so point Vite's root there
// and emit the production bundle to ../dist (referenced by tauri.conf.json).
export default defineConfig({
  root: "src",
  base: "./",
  build: {
    outDir: "../dist",
    emptyOutDir: true,
  },
  server: {
    port: 5173,
    strictPort: true,
  },
});

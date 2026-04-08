import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tailwindcss from "@tailwindcss/vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import path from "path";

export default defineConfig({
  plugins: [react(), tailwindcss(), viteSingleFile()],
  resolve: {
    alias: {
      "@persona/shared": path.resolve(__dirname, "../shared"),
    },
    dedupe: ["react", "react-dom", "react/jsx-runtime"],
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    assetsInlineLimit: 100000,
    cssCodeSplit: false,
  },
});

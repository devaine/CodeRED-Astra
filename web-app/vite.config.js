import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import jsconfigPaths from "vite-jsconfig-paths";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), jsconfigPaths()],
  server: {
    host: '0.0.0.0',
    port: 3000,
    proxy: {
      '/api': {
        target: process.env.RUST_ENGINE_URL || 'http://localhost:8000',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, '')
      }
    }
  },
  preview: {
    host: '0.0.0.0',
    port: 3000
  },
  build: {
    outDir: 'dist'
  }
});

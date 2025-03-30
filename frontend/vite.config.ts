import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { nodePolyfills } from "vite-plugin-node-polyfills";
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    react(),
    nodePolyfills({
      // Whether to polyfill specific globals
      globals: {
        global: true,
        process: true,
        Buffer: true,
      },
      // Whether to polyfill `node:` protocol imports
      protocolImports: true,
    }),
    tailwindcss()
  ],
});

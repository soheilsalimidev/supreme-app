import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Components from "unplugin-vue-components/vite";
import { HeadlessUiResolver } from "unplugin-vue-components/resolvers";
import { HeadlessUiFloatResolver } from "@headlessui-float/vue";
import Icons from "unplugin-icons/vite";
import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";

import { ViteImageOptimizer } from "vite-plugin-image-optimizer";
export default defineConfig(async () => ({
  plugins: [
    vue(),
    Components({
      resolvers: [HeadlessUiResolver(), HeadlessUiFloatResolver()],
    }),
    Icons({}),
    VueI18nPlugin({
      include: [new URL("./src/locales/**", import.meta.url).pathname],
    }),
    ViteImageOptimizer({
      includePublic:true
    }),
  ],
  resolve: {
    alias: {
      "@": "/src",
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));

import { UserConfigExport, defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default defineConfig(({ command, mode, ssrBuild }) => {
  const config: UserConfigExport = {
    plugins: [svelte()],
  };

  if (command == "build") {
    config.build = {
      outDir: "../weather-server/src/static/assets",
      emptyOutDir: true,
      lib: {
        entry: "src/Try.svelte",
        name: "Try",
        formats: ["es"],
        fileName: "try",
      },
      rollupOptions: {
        output: {
          assetFileNames: "try.[ext]",
        },
      },
    };
  }

  return config;
});

import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      assets: "dist",
      pages: "dist",
    }),
  },
  vitePlugin: {
    dynamicCompileOptions: (data) => {
      if (data.filename.includes("node_modules")) {
        return { runes: undefined };
      }

      return { runes: true };
    },
  },
};

export default config;

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
  // csp: {
  //   mode: "auto",
  //   directives: {
  //     "default-src": ["'self'"],
  //     "script-src": ["'self'"],
  //     "style-src": ["'self' 'unsafe-inline'"],
  //     "img-src": ["'self'", "data:"],
  //     "connect-src": ["'self'"],
  //     "font-src": ["'self'"],
  //     "object-src": ["'none'"],
  //     "base-uri": ["'self'"],
  //     "form-action": ["'self'"],
  //   },
  // },
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

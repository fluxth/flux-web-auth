import adapter from "@sveltejs/adapter-node";
import preprocess from "svelte-preprocess";
import svg from "@poppanator/sveltekit-svg";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],

  kit: {
    adapter: adapter({ out: "./build" }),
    vite: {
      plugins: [svg()],
      server: { fs: { allow: ["./static", "./proto"] } },
    },
  },
};

export default config;

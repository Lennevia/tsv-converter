import adapter from '@sveltejs/adapter-static'
import preprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocess({ postcss: true }),
  kit: {
    adapter: adapter(),
    alias: {
      $components: 'src/components',
      $stores: 'src/stores'
    }
  }
}

export default config

// src/routes/+layout.js
// export const csr = false
// export const prerender = true
// export const ssr = false

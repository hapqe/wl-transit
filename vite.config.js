import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    // https://wienerlinien.at/ogd_realtime/monitor should be accessible via /ogd_realtime/monitor
    // from the browser, but not from the server.
    cors: true,
    proxy: {
      '/ogd_realtime/monitor': {
        target: 'https://www.wienerlinien.at',
        changeOrigin: true,
      }
    }
  }
})

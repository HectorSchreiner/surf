import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 3000,
    proxy: {
      "/api": "http://localhost:4000",
    }
  },
  build: {
    target: 'esnext',
  },
});

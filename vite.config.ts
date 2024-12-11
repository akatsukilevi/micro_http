import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    outDir: 'static/bundle',
    assetsDir: '',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: 'views/main.js'
      },
      output: {
        manualChunks: {
          alpine: ['alpinejs'],
          htmx: ['htmx.org'],
          leaflet: ['leaflet']
        },
        entryFileNames: '[name].js',
        chunkFileNames: '[name].js',
        assetFileNames: '[name][extname]',
      }
    }
  },
  publicDir: false,
});

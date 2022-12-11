import { defineConfig, FSWatcher } from 'vite'
import react from '@vitejs/plugin-react'
import basicSsl from '@vitejs/plugin-basic-ssl'
import fs from 'fs';

// https://vitejs.dev/config/
export default defineConfig({
  define: {
    global: 'window',
  },
  plugins: [
    basicSsl(),
    react()
  ],
   resolve: {
    alias: {
      "readable-stream": "vite-compatible-readable-stream"
    },
  },
})

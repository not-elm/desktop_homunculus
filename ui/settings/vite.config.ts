import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import {viteSingleFile} from 'vite-plugin-singlefile'
import tailwindcss from "@tailwindcss/vite";
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
    plugins: [react(), tailwindcss(), viteSingleFile()],
    build: {
        outDir: "../../assets/mods/settings",
    },
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
        },
    },
})

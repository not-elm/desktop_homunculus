import {defineConfig} from 'vite'
import react from '@vitejs/plugin-react'
import {viteSingleFile} from 'vite-plugin-singlefile'
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
    plugins: [react(), tailwindcss(), viteSingleFile()],
    build: {
        outDir: "../../assets/mods/menu",
    }
})

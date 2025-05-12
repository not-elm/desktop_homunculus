import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { copyFileSync } from "node:fs";
import fg from 'fast-glob'
import tailwindcss from '@tailwindcss/vite'
import * as path from "node:path";
import dts from "vite-plugin-dts";

export default defineConfig({
    resolve: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
        },
    },
    plugins: [
        react(),
        tailwindcss(),
        makeFlatPackageInDist(),
        dts({
            outDir: 'dist',
            insertTypesEntry: true,
            rollupTypes: true,
        }),
    ],
    build: {
        outDir: 'dist', // default の設定と同じ
        lib: {
            entry: 'src/index.ts',
            name: '@homunculus/core',
            fileName: 'index',
            formats: ['es', 'umd'], // default の設定と同じ
        },
        rollupOptions: {
            external: ['react', 'react-dom'],
            output: {
                globals: {
                    react: 'React',
                    'react-dom': 'ReactDOM',
                },
            },
        },
    },
})

function makeFlatPackageInDist() {
    return {
        name: 'makeFlatPackageInDist',
        writeBundle() {
            fg.sync('(LICENSE*|*.md|package.json)').forEach((f) =>
                copyFileSync(f, `dist/${f}`)
            )
        }
    }
}
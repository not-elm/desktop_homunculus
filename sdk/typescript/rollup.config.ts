import typescript from '@rollup/plugin-typescript';
import { nodeResolve } from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import terser from '@rollup/plugin-terser';
import { defineConfig } from 'rollup'
// import fg from 'fast-glob'
// import { copyFileSync } from 'fs'
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { dts } from "rollup-plugin-dts";

export default defineConfig([
    {
        input: path.join(path.dirname(fileURLToPath(import.meta.url)), 'src', 'index.ts'),
        output: [
            {
                format: 'esm',
                dir: './dist',
                preserveModules: true,
                preserveModulesRoot: 'src',
                entryFileNames: (chunkInfo) => {
                    if (chunkInfo.name.includes('node_modules')) {
                        return chunkInfo.name.replace('node_modules', 'external') + '.js'
                    }

                    return '[name].js'
                }
            },
            {
                format: 'cjs',
                dir: './dist',
                preserveModules: true,
                preserveModulesRoot: 'src',
                entryFileNames: (chunkInfo) => {
                    if (chunkInfo.name.includes('node_modules')) {
                        return chunkInfo.name.replace('node_modules', 'external') + '.cjs'
                    }

                    return '[name].cjs'
                }
            },
        ],
        plugins: [
            typescript({
                declaration: true,
                declarationDir: 'dist/types',
                rootDir: 'src',
                target: "esnext",
                module: "esnext",
            }),
            // makeFlatPackageInDist()
        ],
    },
    {
        input: 'dist/types/index.d.ts',
        output: { file: 'dist/index.d.ts', format: 'es' },
        plugins: [dts()]
    },
    {
        input: './src/index.ts',
        output: {
            file: '../../assets/scripts/denoMain.js',
            format: 'iife',
            name: '__IIFE__',
            footer: 'Object.defineProperty(Deno, "api", { value: __IIFE__ })',
        },
        plugins: [
            nodeResolve(),
            commonjs(),
            typescript({
                target: "esnext",
                module: "esnext",
                declaration: false,
            }),
            terser()
        ]
    },
    {
        input: './src/index.ts',
        output: {
            file: '../../crates/homunculus_api/src/webview/api.js',
            format: 'iife',
            name: '__IIFE__',
            footer: 'Object.defineProperty(Window, "api", { value: __IIFE__ })',
        },
        plugins: [
            nodeResolve(),
            commonjs(),
            typescript({
                target: "esnext",
                module: "esnext",
                declaration: false,
            }),
            terser()
        ]
    }
]);

// function makeFlatPackageInDist() {
//     return {
//         name: 'makeFlatPackageInDist',
//         writeBundle() {
//             fg.sync('(LICENSE*|*.md|package.json)').forEach((f) =>
//                 copyFileSync(f, `dist/${f}`)
//             )
//         }
//     }
// }
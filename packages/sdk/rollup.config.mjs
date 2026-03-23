import typescript from '@rollup/plugin-typescript';
import {defineConfig} from 'rollup'
import * as path from "node:path";
import {fileURLToPath} from "node:url";
import {dts} from "rollup-plugin-dts";
import {rimrafSync} from 'rimraf';

const addJsExtension = {
    name: 'add-js-extension',
    generateBundle(_options, bundle) {
        for (const file of Object.values(bundle)) {
            if (file.type === 'chunk' && file.code) {
                file.code = file.code.replace(
                    /(from\s+['"])(\.\/[^'"]+?)(?<!\.js)(?<!\.cjs)(['"])/g,
                    '$1$2.js$3'
                );
                file.code = file.code.replace(
                    /(export\s+\*\s+from\s+['"])(\.\/[^'"]+?)(?<!\.js)(?<!\.cjs)(['"])/g,
                    '$1$2.js$3'
                );
            }
        }
    }
};

const cleanDistTypes = {
    name: 'clean-dist-types',
    closeBundle() {
        rimrafSync('dist/types');
        process.exit(0);
    }
};

const srcDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig([
    {
        input: [
            path.join(srcDir, 'src', 'index.ts'),
            path.join(srcDir, 'src', 'commands.ts'),
            path.join(srcDir, 'src', 'rpc.ts'),
            path.join(srcDir, 'src', 'wake-word-matcher.ts'),
        ],
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
            addJsExtension,
        ],
    },
    {
        input: 'dist/types/index.d.ts',
        output: {file: 'dist/index.d.ts', format: 'es'},
        plugins: [dts()]
    },
    {
        input: 'dist/types/commands.d.ts',
        output: {file: 'dist/commands.d.ts', format: 'es'},
        plugins: [dts()]
    },
    {
        input: 'dist/types/rpc.d.ts',
        output: {file: 'dist/rpc.d.ts', format: 'es'},
        plugins: [dts()]
    },
    {
        input: 'dist/types/rpc-client.d.ts',
        output: {file: 'dist/rpc-client.d.ts', format: 'es'},
        plugins: [dts()]
    },
    {
        input: 'dist/types/wake-word-matcher.d.ts',
        output: {file: 'dist/wake-word-matcher.d.ts', format: 'es'},
        plugins: [dts(), cleanDistTypes]
    },
]);

/// <reference types="vitest/config" />

import { copyFileSync } from 'node:fs';
import * as path from 'node:path';
import { fileURLToPath } from 'node:url';
import { storybookTest } from '@storybook/addon-vitest/vitest-plugin';
import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react-swc';
import { playwright } from '@vitest/browser-playwright';
import fg from 'fast-glob';
import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';

const dirname =
  typeof __dirname !== 'undefined' ? __dirname : path.dirname(fileURLToPath(import.meta.url));

// More info at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon
export default defineConfig({
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src'),
      'vaul/style.css': path.resolve(__dirname, 'node_modules/vaul/style.css'),
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
    outDir: 'dist',
    // default の設定と同じ
    lib: {
      entry: 'src/index.ts',
      name: 'hmcs-ui',
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
  test: {
    projects: [
      {
        extends: true,
        plugins: [
          // The plugin will run tests for the stories defined in your Storybook config
          // See options at: https://storybook.js.org/docs/next/writing-tests/integrations/vitest-addon#storybooktest
          storybookTest({
            configDir: path.join(dirname, '.storybook'),
          }),
        ],
        test: {
          name: 'storybook',
          browser: {
            enabled: true,
            headless: true,
            provider: playwright({}),
            instances: [
              {
                browser: 'chromium',
              },
            ],
          },
          setupFiles: ['.storybook/vitest.setup.ts'],
        },
      },
    ],
  },
});
function makeFlatPackageInDist() {
  return {
    name: 'makeFlatPackageInDist',
    writeBundle() {
      fg.sync('(LICENSE*|*.md|package.json)').forEach((f) => {
        copyFileSync(f, `dist/${f}`);
      });
    },
  };
}

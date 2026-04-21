import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    root: new URL('.', import.meta.url).pathname,
    include: ['src/**/*.test.ts', 'src/**/*.test.tsx'],
    environment: 'node',
    globals: false,
  },
});

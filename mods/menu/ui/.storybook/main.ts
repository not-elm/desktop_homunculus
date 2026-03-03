import type { StorybookConfig } from "@storybook/react-vite";
import tailwindcss from "@tailwindcss/vite";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

const config: StorybookConfig = {
  stories: ["../src/**/*.stories.@(ts|tsx)"],
  addons: [],
  framework: "@storybook/react-vite",
  typescript: {
    reactDocgen: false,
  },
  core: {
    disableTelemetry: true,
  },
  viteFinal(config) {
    config.resolve ??= {};
    const existingAlias = Array.isArray(config.resolve.alias)
      ? config.resolve.alias
      : Object.entries(config.resolve.alias ?? {}).map(([find, replacement]) => ({
          find,
          replacement,
        }));
    config.resolve.alias = [
      ...existingAlias,
      { find: "@hmcs/sdk", replacement: resolve(__dirname, "../src/__mocks__/homunculus-api.ts") },
      { find: "@hmcs/ui/storybook", replacement: resolve(__dirname, "../../../../packages/ui/src/storybook/preview.ts") },
      { find: "@hmcs/ui/dist/index.css", replacement: resolve(__dirname, "../../../../packages/ui/dist/index.css") },
      { find: /^@hmcs\/ui$/, replacement: resolve(__dirname, "./hmcs-ui-shim.ts") },
      { find: "@", replacement: resolve(__dirname, "../../../../packages/ui/src") },
    ];

    config.plugins ??= [];
    config.plugins.push(tailwindcss());

    config.optimizeDeps ??= {};
    config.optimizeDeps.include = [
      ...(config.optimizeDeps.include ?? []),
      "react",
      "react-dom",
      "react/jsx-runtime",
    ];

    return config;
  },
};

export default config;

---
title: "Setup & Build"
sidebar_position: 2
---

# Setup & Build

Build your first WebView UI for a Desktop Homunculus MOD. By the end of this guide, you will have a styled "Hello from WebView" card rendering inside the engine.

## Prerequisites

- An existing MOD (or follow [Quick Start](../quick-start.md) first)
- Node.js 22 or later
- pnpm
- Desktop Homunculus running

## Step 1 -- Create the UI Project

Inside your mod directory, create a `ui/` folder with a `src/` subdirectory and initialize it as a package:

```bash
mkdir -p ui/src
cd ui
pnpm init
```

## Step 2 -- Install Dependencies

Install React, the shared component library, and the build toolchain:

```bash
pnpm add react react-dom @hmcs/ui
pnpm add -D @vitejs/plugin-react-swc @tailwindcss/vite tailwindcss typescript vite vite-plugin-singlefile @types/react @types/react-dom
```

:::note Why `vite-plugin-singlefile`?
Desktop Homunculus loads webview HTML from a single file asset, so all CSS and JavaScript must be inlined into one HTML file. The `vite-plugin-singlefile` plugin handles this automatically during the build.
:::

## Step 3 -- Configure TypeScript

Create `ui/tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true
  },
  "include": ["src"]
}
```

## Step 4 -- Configure Vite

Create `ui/vite.config.ts`:

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import tailwindcss from "@tailwindcss/vite";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig({
  plugins: [react(), tailwindcss(), viteSingleFile()],
  resolve: {
    dedupe: ["react", "react-dom", "react/jsx-runtime"],
  },
  build: {
    outDir: "dist",
    emptyOutDir: true,
    assetsInlineLimit: 100000,
    cssCodeSplit: false,
  },
});
```

Key settings:

- **`viteSingleFile()`** -- bundles everything into one HTML file so the engine can load it as a single asset.
- **`dedupe`** -- prevents duplicate React instances when `@hmcs/ui` also depends on React.
- **`assetsInlineLimit: 100000`** -- inlines assets up to 100 KB as data URLs instead of emitting separate files.
- **`cssCodeSplit: false`** -- keeps all CSS in one chunk so nothing is lost during single-file inlining.

## Step 5 -- Create the Entry Files

### `ui/index.html`

```html
<!DOCTYPE html>
<html lang="en" class="dark">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>My WebView</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

The `class="dark"` attribute enables dark mode. WebViews always use dark mode to match the engine's transparent overlay aesthetic.

### `ui/src/index.css`

```css
@import "tailwindcss";
@import "@hmcs/ui/dist/index.css";

body {
  background: transparent;
}

#root {
  width: 100%;
  height: 100%;
}
```

`background: transparent` lets the Bevy window show through. The `@hmcs/ui` import brings in the glassmorphism design system and component styles.

### `ui/src/main.tsx`

```tsx
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import "./index.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>
);
```

## Step 6 -- Build the App

Create `ui/src/App.tsx`:

```tsx
import { Card, CardHeader, CardTitle, CardContent } from "@hmcs/ui";

export function App() {
  return (
    <div className="p-4">
      <Card>
        <CardHeader>
          <CardTitle>Hello from WebView</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-muted-foreground">
            This UI is rendered inside Desktop Homunculus.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
```

## Step 7 -- Register as a Mod Asset

Open your mod's root `package.json` and add a build script and the asset declaration:

```json
{
  "name": "my-mod",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "build": "vite build ui"
  },
  "homunculus": {
    "assets": {
      "my-mod:ui": {
        "path": "ui/dist/index.html",
        "type": "html",
        "description": "My WebView UI"
      }
    }
  },
  "dependencies": {
    "@hmcs/sdk": "...",
    "@hmcs/ui": "...",
    "react": "...",
    "react-dom": "..."
  }
}
```

The `"build"` script tells Vite to build the `ui/` directory. The `homunculus.assets` entry registers the built HTML file so the engine can load it by asset ID.

## Step 8 -- Build and Test

```bash
pnpm build
hmcs mod install /path/to/my-mod
```

Restart Desktop Homunculus. The webview can be opened programmatically via:

```typescript
import { Webview, webviewSource } from "@hmcs/sdk";

Webview.open({ source: webviewSource.local("my-mod:ui") });
```

## Going Further

### Accessing the SDK Inside a WebView

Use `Webview.current()` to get a handle to the webview from within your React app, then call `linkedVrm()` to access the associated character:

```typescript
import { Webview } from "@hmcs/sdk";

const webview = Webview.current();
const vrm = await webview?.linkedVrm();
```

`Webview.current()` reads the `window.WEBVIEW_ENTITY` value that CEF injects into every webview context.

### Opening via a MOD Command

Create a MOD command to open the webview on demand. Add `commands/open-ui.ts`:

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { z } from "zod";
import { Webview, webviewSource } from "@hmcs/sdk";
import { input } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(
    z.object({ linkedVrm: z.number() })
  );
  await Webview.open({
    source: webviewSource.local("my-mod:ui"),
    size: [1, 0.9],
    viewportSize: [900, 700],
    offset: [1.1, 0],
    linkedVrm: data.linkedVrm,
  });
} catch (e) {
  console.error(e);
}
```

Register it in your `package.json`:

```json
"bin": {
  "open-ui": "commands/open-ui.ts"
}
```

### Adding a Right-Click Menu Entry

To let users open your UI from the right-click context menu, add a `menus` entry in your `package.json` under the `homunculus` field:

```json
"menus": [
  {
    "id": "open-my-ui",
    "text": "Open My UI",
    "command": "open-ui"
  }
]
```

The `command` value matches the key in `"bin"`, so clicking the menu entry runs the `open-ui` MOD command.

### Development Workflow

Use `pnpm dev` inside the `ui/` directory to start a Vite dev server and iterate in a browser. SDK calls will fail outside the engine, but you can work on layout, styling, and component structure without rebuilding.

When you are ready to test inside the engine:

```bash
pnpm build
hmcs mod install /path/to/my-mod
```

Then restart Desktop Homunculus to see the result.

:::tip Development Tips
- Press `F1`/`F2` to open/close DevTools in a running webview
- `Cmd+[` / `Cmd+]` to navigate back/forward
- `viewportSize` controls HTML pixel dimensions (e.g., `[800, 600]`); `size` controls 3D world space dimensions (e.g., `[0.7, 0.7]`)
:::

## Next Steps

- **[Component Library](./component-library)** -- Learn about the `@hmcs/ui` components available for your WebView UI

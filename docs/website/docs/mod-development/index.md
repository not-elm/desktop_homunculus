---
title: "What is a MOD?"
sidebar_position: 1
---

# What is a MOD?

MODs are add-on packages that extend Desktop Homunculus with new characters, behaviors, UI panels, and integrations. Whether you want to add a custom 3D avatar, connect a text-to-speech service, or build an interactive settings panel, MODs are how you do it.

MODs are standard npm packages. You can publish them to the npm registry, share them with others, and install them with a single command. No special tooling or custom formats required -- if you know how to build an npm package, you already know the basics.

## How MODs Work

Every MOD is a **pnpm package** with a special `homunculus` field in its `package.json`. This field declares the MOD's assets (3D models, animations, sounds, HTML files) and optional menu entries.

When Desktop Homunculus launches, the engine discovers installed MODs by running `pnpm ls` in the mods directory (`~/.homunculus/mods/`) and reading each MOD's `package.json`. MODs are installed with the `hmcs mod install` command. Each MOD can declare:

- A **service** (`homunculus.service` field) -- a long-running Node.js child process that runs automatically when the app starts
- **MOD commands** (`bin` field) -- invoked through the HTTP API when needed
- **Assets** (`homunculus.assets` field) -- files bundled with the MOD (VRM models, animations, sounds, UI)

MODs communicate with the engine through a local **HTTP API** running on `localhost:3100`. The TypeScript SDK (`@hmcs/sdk`) wraps this API with a high-level, type-safe interface. Scripts run via `tsx`, so you can write TypeScript directly without a build step.

## What Can a MOD Do?

MODs combine any mix of the following capabilities. A single MOD can do one of these things or all of them at once.

- **Spawn characters** — Load a VRM 3D model and control its animations, expressions, and behavior. The `@hmcs/persona` MOD spawns personas that idle on your desktop and react to dragging.

- **Run a service** — Run a long-running TypeScript process when the app launches (declared via the `homunculus.service` field). Services typically set up characters and event listeners. The `@hmcs/menu` MOD uses a service to initialize the right-click menu overlay.

- **Expose MOD commands** — Provide commands other MODs or AI agents can invoke through the HTTP API (declared via the `bin` field). For example, the `@hmcs/voicevox` MOD exposes `voicevox:speak` and `voicevox:speakers` commands for text-to-speech.

- **Embed UI panels** — Bundle a WebView-based interface (React + Vite) as an HTML asset. The `@hmcs/persona` MOD combines an HTML asset, a `bin` command to open the panel, and a menu entry — showing how capabilities work together.

- **Add menu entries** — Register items in the right-click context menu that trigger commands or open webviews (declared via `homunculus.menus`).

- **Bundle assets** — Package VRM models, animations (VRMA), sounds, images, and HTML files that other MODs can reference by asset ID.

## Quick Example

Here is the `package.json` for a MOD that loads a VRM character:

```json
{
  "name": "@hmcs/my-character",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@hmcs/sdk": "workspace:*"
  },
  "homunculus": {
    "service": "index.ts",
    "assets": {
      "vrm:elmer": {
        "path": "assets/Elmer.vrm",
        "type": "vrm",
        "description": "VRM model named Elmer"
      }
    }
  }
}
```

The `homunculus.service` field points to a TypeScript service that uses the SDK to spawn the character and set up its behavior. The `homunculus.assets` field registers a VRM model with the asset ID `vrm:elmer`.

:::note
This is an illustrative example. Asset IDs follow `type:name` format (e.g. `vrm:elmer`, `vrma:idle-maid`, `se:open`). The built-in `@hmcs/assets` MOD provides default animations and sound effects.
:::

## Get Started

- **[Quick Start](./quick-start.md)** -- Build your first MOD from scratch in 5 minutes
- **[Package Configuration](./project-setup/package-json.md)** -- Full reference for `package.json` and the `homunculus` field
- **[Asset IDs](./project-setup/asset-ids.md)** -- How asset identifiers work across the MOD system

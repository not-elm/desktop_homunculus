---
title: "SDK Quick Start"
sidebar_position: 2
---

# SDK Quick Start

Get started with `@hmcs/sdk` -- installation, your first service, and the commands entry point for bin scripts.

:::tip
This page covers the SDK itself. For a full end-to-end guide to creating a MOD (project setup, `package.json`, assets, installation), see the [MOD Quick Start](../quick-start).
:::

## Installation

```bash
pnpm add @hmcs/sdk
```

The SDK requires **Node.js 22 or later**. MOD scripts run directly as TypeScript via `node --experimental-strip-types` -- no build step needed.

## Your First Script

A MOD's `main` script runs automatically when Desktop Homunculus starts. Create `index.ts` in your MOD's root:

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

// Spawn a VRM character using an asset declared in package.json
const character = await Vrm.spawn("my-mod:character");

// Play the built-in idle animation on loop
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// Make the character's eyes follow the mouse cursor
await character.lookAtCursor();

// Listen for state changes (drag, idle, sitting)
character.events().on("state-change", async (e) => {
  console.log("State changed to:", e.state);
});
```

Set `"main": "index.ts"` in your `package.json` so the engine knows which file to run at startup.

## Key Concepts

### Asset IDs

Assets are referenced by globally unique string IDs in the format `"mod-name:asset-name"`. For example:

- `"my-mod:character"` -- a VRM model declared by `my-mod`
- `"vrma:idle-maid"` -- a VRMA animation from the built-in `@hmcs/assets` MOD

Assets are declared in your MOD's `package.json` under the `homunculus.assets` field. See [Asset IDs](../project-setup/asset-ids) for details.

### HTTP API

The SDK wraps the engine's HTTP REST API running at `localhost:3100`. Each SDK module (`Vrm`, `entities`, `audio`, etc.) translates method calls into HTTP requests. You rarely need to call the API directly, but it is available via the `host` module for advanced use cases.

### Event-Driven Patterns

MODs react to real-time events using two mechanisms:

- **VRM events** -- pointer clicks, drag, state changes, animation events (via `vrm.events()`)
- **Signals** -- cross-process pub/sub messaging for communication between MOD scripts and WebViews

## Commands Entry Point

`@hmcs/sdk/commands` is a **separate entry point** for bin scripts (on-demand commands declared in `package.json` under `"bin"`). It provides stdin parsing and structured output helpers. See the [Commands](./commands) page for the full API reference.

:::warning
Do **not** import `@hmcs/sdk/commands` from a MOD's main script or from browser-side code. It uses `process.stdin`, which is only available in Node.js bin scripts.
:::

## Next Steps

- **[SDK Overview](./)** -- Full module map with all 16 modules
- **[MOD Quick Start](../quick-start)** -- End-to-end MOD creation tutorial

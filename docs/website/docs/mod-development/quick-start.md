---
title: "Quick Start"
sidebar_position: 2
---

# Quick Start

Build your first Desktop Homunculus MOD from scratch. By the end of this guide, you will have a working MOD that creates a persona, attaches a VRM character, plays an idle animation, and responds to user interactions.

## Prerequisites

Before you begin, make sure you have:

- **Node.js 22 or later** -- required for TypeScript support via tsx
- **pnpm** -- the package manager used by the MOD system
- **Desktop Homunculus** running on your machine
- **`hmcs` CLI** installed globally (see [Installation](/getting-started/installation))

:::tip
Run `node -v` and `hmcs --version` to verify your setup before continuing.
:::

## Step 1: Create the Project

Create a new directory and initialize it as an npm package:

```bash
mkdir my-character
cd my-character
pnpm init
```

Install the SDK:

```bash
pnpm add @hmcs/sdk
```

`@hmcs/sdk` provides the TypeScript API for controlling characters, playing sounds, and more. If your MOD uses built-in animations (like `vrma:idle-maid`), you must also install `@hmcs/assets`: `hmcs mod install @hmcs/assets`.

## Step 2: Configure package.json

Open `package.json` and add the `homunculus` field along with the `type` field. You will also need a VRM model file -- place it in an `assets/` directory inside your project.

```json
{
  "name": "my-character",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@hmcs/sdk": "..."
  },
  "homunculus": {
    "service": "service.ts",
    "assets": {
      "my-character:vrm": {
        "path": "assets/MyModel.vrm",
        "type": "vrm",
        "description": "My custom VRM character"
      }
    }
  }
}
```

## Step 3: Write the Service

Create `service.ts` in the project root. This script runs automatically when Desktop Homunculus starts.

```typescript
import { persona, repeat } from "@hmcs/sdk";

// Create a persona and attach the VRM character
const character = await persona.create({ id: "my-character" });
const vrm = await character.attachVrm("my-character:vrm");

// Play the built-in idle animation on loop
const animationOptions = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

await vrm.playVrma({
  asset: "vrma:idle-maid",
  ...animationOptions,
});

// Make the character follow your cursor
await vrm.lookAtCursor();

// React to state changes (drag, idle, sitting)
character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await vrm.playVrma({
      asset: "vrma:idle-maid",
      ...animationOptions,
    });
    await vrm.lookAtCursor();
  } else if (e.state === "drag") {
    await vrm.unlook();
    await vrm.playVrma({
      asset: "vrma:grabbed",
      ...animationOptions,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await vrm.playVrma({
      asset: "vrma:idle-sitting",
      ...animationOptions,
    });
    await vrm.lookAtCursor();
  }
});
```

This script does three things:

1. **Creates** a persona and **attaches** the VRM model registered as `my-character:vrm`
2. **Plays** the built-in `vrma:idle-maid` animation on a loop
3. **Listens** for state changes to switch animations when the user drags or drops the character

## Step 4: Install and Test

Install your MOD using the `hmcs` CLI:

```bash
hmcs mod install /path/to/my-character
```

Restart Desktop Homunculus. Your character should appear on the desktop. Try dragging it to see the animation change.

## Step 5: Iterate

When you make changes to your MOD:

1. Run `hmcs mod install /path/to/my-character` again to update the installed copy
2. Restart Desktop Homunculus to pick up the changes

## Next Steps

- **[Package Configuration](./project-setup/package-json.md)** -- Learn about all the fields in `package.json`, including `bin` commands and menus
- **[Asset IDs](./project-setup/asset-ids.md)** -- Understand how asset identifiers work across MODs
- **[What is a MOD?](./index.md)** -- Learn what capabilities MODs can provide

---
title: "SDK Overview"
sidebar_position: 1
---

# SDK Overview

`@hmcs/sdk` is the official TypeScript SDK for building Desktop Homunculus MODs. It wraps the engine's HTTP API (`localhost:3100`) with type-safe methods, real-time event streaming, and high-level abstractions for character control, audio, UI, and more.

## Installation

```shell
pnpm add @hmcs/sdk
```

:::info[Node.js Requirement]
Desktop Homunculus MODs require **Node.js 22 or later** to run TypeScript files directly using `--experimental-strip-types`. No build step is needed for MOD scripts.
:::

## Module Map

The SDK is organized into 17 modules, all available from the main `@hmcs/sdk` entry point, plus a separate `@hmcs/sdk/commands` entry point for bin script utilities.

| Module | Import | Description |
|---|---|---|
| **Vrm** | `import { Vrm } from "@hmcs/sdk"` | Spawn, find, animate, and control VRM 3D characters. The core SDK module. |
| **entities** | `import { entities } from "@hmcs/sdk"` | Query and manipulate ECS entities -- find by name, get/set transforms, tween animations. |
| **audio** | `import { audio } from "@hmcs/sdk"` | Play sound effects (`audio.se`) and background music (`audio.bgm`) with fade/volume controls. |
| **Webview** | `import { Webview } from "@hmcs/sdk"` | Open and manage embedded HTML interfaces in 3D space, linked to characters or free-floating. |
| **signals** | `import { signals } from "@hmcs/sdk"` | Cross-process pub/sub communication via Server-Sent Events (SSE). |
| **preferences** | `import { preferences } from "@hmcs/sdk"` | Persistent key-value storage with JSON serialization for user settings and MOD data. |
| **effects** | `import { effects } from "@hmcs/sdk"` | Trigger visual stamp effects on screen (images with position, size, and duration). |
| **displays** | `import { displays } from "@hmcs/sdk"` | Query connected monitors -- dimensions, positions, and frame rectangles. |
| **coordinates** | `import { coordinates } from "@hmcs/sdk"` | Convert between screen-space (viewport) and 3D world-space coordinates. |
| **speech** | `import { speech } from "@hmcs/sdk"` | Utilities for converting phoneme data into timeline keyframes for lip-sync. |
| **app** | `import { app } from "@hmcs/sdk"` | Application lifecycle -- health checks, platform info, engine version, loaded MODs. |
| **mods** | `import { mods } from "@hmcs/sdk"` | List installed MODs, execute bin commands, stream command output, query menus. |
| **assets** | `import { assets } from "@hmcs/sdk"` | Query the asset registry -- list assets by type (`vrm`, `vrma`, `sound`, `image`, `html`) or MOD. |
| **shadowPanel** | `import { shadowPanel } from "@hmcs/sdk"` | Control the shadow overlay panel transparency for atmospheric effects. |
| **host** | `import { host } from "@hmcs/sdk"` | Low-level HTTP client for direct API calls. Used internally by all other modules. |
| **Math types** | `import { type Transform, type Vec3 } from "@hmcs/sdk"` | Transform, Vec2, Vec3, Quat, and Rect type definitions. |
| **utils** | `import { sleep } from "@hmcs/sdk"` | Utility helpers — `sleep(ms)` for non-blocking delays. |

### Commands Sub-Entry Point

`@hmcs/sdk/commands` is a **separate entry point** for utilities used in bin command scripts. It is intentionally excluded from the main `@hmcs/sdk` import because it depends on Node.js-specific APIs (`process.stdin`).

| Export | Description |
|---|---|
| `input.parse(schema)` | Read JSON from stdin and validate it against a Zod schema. |
| `input.parseMenu()` | Parse menu command stdin and return a `Vrm` instance for the linked character. |
| `input.read()` | Read all of stdin as a raw UTF-8 string. |
| `output.succeed(data)` | Write JSON result to stdout and exit with code 0. |
| `output.fail(code, message)` | Write structured error to stderr and exit with non-zero code. |
| `output.write(data)` | Write JSON result to stdout (without exiting). |
| `output.writeError(code, message)` | Write structured error to stderr (without exiting). |
| `StdinParseError` | Error class thrown when stdin parsing or validation fails. |

See the [Commands](./commands) page for detailed API documentation.

```typescript
// Import from the separate entry point
import { input } from "@hmcs/sdk/commands";
import { z } from "zod";

const data = await input.parse(
  z.object({
    speaker: z.number().default(0),
    text: z.string(),
  })
);
```

:::warning
Do **not** import `@hmcs/sdk/commands` from within a MOD's main script or from browser-side code. It uses `process.stdin`, which is only available in Node.js bin scripts.
:::

## Quick Example

```typescript
import { Vrm, preferences, repeat } from "@hmcs/sdk";

// Load saved position, then spawn a VRM character
const transform = await preferences.load("transform::my-mod:vrm");
const character = await Vrm.spawn("my-mod:character", { transform });

// Play a looping idle animation
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// Make the character follow the mouse cursor
await character.lookAtCursor();

// Listen for state changes (idle, drag, sitting, etc.)
character.events().on("state-change", async (e) => {
  console.log("State changed to:", e.state);
});
```

## Next Steps

- **[VRM Module](./vrm/)** -- Deep dive into spawning characters, playing animations, handling events, and speech.

---
title: "Assets"
sidebar_position: 2
---

# Assets

The Assets MOD (`@hmcs/assets`) provides the default resources that other MODs depend on — a VRM character model, VRMA animations, and sound effects.

## Overview

| Asset ID | Type | Description |
|---|---|---|
| `vrm:elmer` | VRM | Default character model (Elmer) |
| `vrma:idle-maid` | VRMA | Standing idle animation with hands clasped in front |
| `vrma:grabbed` | VRMA | Reactive pose while being dragged by the user |
| `vrma:idle-sitting` | VRMA | Seated idle loop |
| `se:open` | Sound | HUD open sound effect |
| `se:close` | Sound | HUD close sound effect |

## Features

These assets are referenced by their asset ID in other MODs and SDK calls. For example, the Elmer MOD spawns the default character with:

```ts
const elmer = await Vrm.spawn("vrm:elmer");
```

MOD developers can reference these assets in their own MODs using the same IDs. See the [SDK documentation](/docs/mod-development/sdk) for details.

## Notes

- This MOD has no startup script — it only provides static asset files.
- The `@hmcs/elmer` MOD depends on this MOD — it uses `vrm:elmer`, `vrma:idle-maid`, `vrma:grabbed`, and `vrma:idle-sitting` to spawn and animate the default character.
- Removing this MOD will break the Elmer MOD and any other MODs that reference these asset IDs.

---
title: "setVrmaSpeed"
sidebar_position: 20
---

# setVrmaSpeed

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setVrmaSpeed(asset, speed)` changes the playback speed of an animation.

```typescript
// Slow motion
await character.setVrmaSpeed("vrma:idle-maid", 0.5);

// Double speed
await character.setVrmaSpeed("vrma:idle-maid", 2.0);

// Normal speed
await character.setVrmaSpeed("vrma:idle-maid", 1.0);
```

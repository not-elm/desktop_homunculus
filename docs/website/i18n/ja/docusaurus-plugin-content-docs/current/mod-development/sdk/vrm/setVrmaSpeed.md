---
title: "setVrmaSpeed"
sidebar_position: 20
---

# setVrmaSpeed

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setVrmaSpeed(asset, speed)` はアニメーションの再生速度を変更します。

```typescript
// スローモーション
await character.setVrmaSpeed("vrma:idle-maid", 0.5);

// 2倍速
await character.setVrmaSpeed("vrma:idle-maid", 2.0);

// 通常速度
await character.setVrmaSpeed("vrma:idle-maid", 1.0);
```

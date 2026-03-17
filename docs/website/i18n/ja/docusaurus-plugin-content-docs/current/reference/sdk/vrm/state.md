---
title: "state"
sidebar_position: 12
---

# state

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.state()` はキャラクターの現在の状態文字列（例：`"idle"`、`"drag"`、`"sitting"`）を返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const state = await character.state();
console.log("Current state:", state);
```

状態を変更するには [`setState`](./setState) を使用し、状態が変わったときに反応するには [`events()`](./events) を購読してください。

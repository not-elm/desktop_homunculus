---
title: "lookAtCursor"
sidebar_position: 35
---

# lookAtCursor

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.lookAtCursor()` はキャラクターの目を画面上のマウスカーソルに追従させます。

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.lookAtCursor();
```

典型的なパターンは、キャラクターがアイドル状態のときにカーソル追従を有効にし、ドラッグなどのインタラクション中は無効にすることです：

```typescript
character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
  }
});
```

視線追従動作を無効にするには [`unlook`](./unlook) を使用し、特定のエンティティを追跡するには [`lookAtTarget`](./lookAtTarget) を使用してください。

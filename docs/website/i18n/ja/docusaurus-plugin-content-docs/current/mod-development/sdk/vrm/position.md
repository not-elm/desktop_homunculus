---
title: "position"
sidebar_position: 11
---

# position

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.position()` はキャラクターの位置を画面座標とワールド座標の両方で返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const pos = await character.position();

// 画面座標（マルチモニターのグローバルビューポート）
if (pos.globalViewport) {
  console.log(`Screen: (${pos.globalViewport[0]}, ${pos.globalViewport[1]})`);
}

// Bevy ワールド座標
console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
```

[`PositionResponse`](./types) を返します。キャラクターが現在画面に表示されていない場合、`globalViewport` フィールドは `null` になります。

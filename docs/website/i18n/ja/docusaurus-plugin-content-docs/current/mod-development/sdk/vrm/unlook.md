---
title: "unlook"
sidebar_position: 37
---

# unlook

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.unlook()` は視線追従動作を完全にオフにします。キャラクターの目はデフォルトのアニメーション駆動状態に戻ります。

```typescript
await character.unlook();
```

カーソル追従を再有効にするには [`lookAtCursor`](./lookAtCursor) を使用し、特定のエンティティを追跡するには [`lookAtTarget`](./lookAtTarget) を使用してください。

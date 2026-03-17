---
title: "Vrm.findAll"
sidebar_position: 5
---

# Vrm.findAll

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAll()` は現在読み込まれているすべてのキャラクターの `Vrm` インスタンスを返します。

```typescript
// 読み込み済みのすべてのキャラクターの Vrm インスタンスを取得
const characters = await Vrm.findAll();
for (const vrm of characters) {
  const name = await vrm.name();
  console.log(`${name} (entity: ${vrm.entity})`);
}
```

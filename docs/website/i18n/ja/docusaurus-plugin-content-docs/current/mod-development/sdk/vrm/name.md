---
title: "name"
sidebar_position: 14
---

# name

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.name()` はこのキャラクターの VRM モデル名を返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const name = await character.name();
console.log(name); // "MyAvatar"
```

この名前は VRM モデルの内部名に対応しており、[`Vrm.findByName`](./findByName) や [`Vrm.waitLoadByName`](./waitLoadByName) で使用する識別子と同じです。

---
title: "Vrm.findByName"
sidebar_position: 3
---

# Vrm.findByName

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findByName(name)` はすでに読み込まれているキャラクターの `Vrm` インスタンスを返します。その名前のキャラクターが存在しない場合はエラーをスローします。

```typescript
try {
  const character = await Vrm.findByName("MyAvatar");
  console.log("Found entity:", character.entity);
} catch (e) {
  console.log("Character not found");
}
```

:::tip
キャラクターがすでに存在していると想定できる場合は `findByName` を使用してください。あなたの MOD がキャラクターの MOD よりも先に開始される場合は [`Vrm.waitLoadByName`](./waitLoadByName) を使用してください。
:::

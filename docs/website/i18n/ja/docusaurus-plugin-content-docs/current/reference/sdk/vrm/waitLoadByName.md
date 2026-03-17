---
title: "Vrm.waitLoadByName"
sidebar_position: 4
---

# Vrm.waitLoadByName

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.waitLoadByName(name)` は指定した名前のキャラクターの読み込みが完了するまでブロックし、`Vrm` インスタンスを返します。あなたの MOD がキャラクターの MOD よりも先に開始される場合に使用してください。

```typescript
// "MyAvatar" が完全に読み込まれるまで待機
const character = await Vrm.waitLoadByName("MyAvatar");
```

:::tip
別の MOD がスポーンするキャラクターに依存する場合は、MOD サービスで `waitLoadByName` を使用してください。キャラクターがすでに存在していると想定できる場合は [`Vrm.findByName`](./findByName) を使用してください。
:::

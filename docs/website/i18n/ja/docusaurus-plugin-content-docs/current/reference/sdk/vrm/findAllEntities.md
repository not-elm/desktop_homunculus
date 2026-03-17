---
title: "Vrm.findAllEntities"
sidebar_position: 6
---

# Vrm.findAllEntities

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAllEntities()` は現在読み込まれているすべての VRM インスタンスの生のエンティティ ID を `Vrm` オブジェクトにラップせずに返します。

```typescript
const entityIds = await Vrm.findAllEntities();
console.log(`Found ${entityIds.length} VRM entities`);
```

エンティティ ID のみが必要で `Vrm` ラッパーオブジェクトの構築のオーバーヘッドを避けたい場合に使用します。インスタンスメソッドを呼び出す必要がある場合は [`Vrm.findAll`](./findAll) を使用してください。

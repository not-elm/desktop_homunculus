---
title: "lookAtTarget"
sidebar_position: 36
---

# lookAtTarget

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.lookAtTarget(entity)` はキャラクターの目を ID で指定した特定のエンティティに向けます。キャラクター同士を見つめ合わせたり、シーン内の特定のオブジェクトを見させたりするのに便利です。

```typescript
const character = await Vrm.findByName("MyAvatar");
const other = await Vrm.findByName("OtherCharacter");

// MyAvatar を OtherCharacter の頭に向ける
const headEntity = await other.findBoneEntity("head");
await character.lookAtTarget(headEntity);
```

VRM のボーンだけでなく、任意の Bevy ECS エンティティのエンティティ ID を使用することもできます：

```typescript
// 別の VRM のルートエンティティを見る
await character.lookAtTarget(other.entity);
```

## 例：キャラクター同士の視線

```typescript
const alice = await Vrm.findByName("Alice");
const bob = await Vrm.findByName("Bob");

const aliceHead = await alice.findBoneEntity("head");
const bobHead = await bob.findBoneEntity("head");

await alice.lookAtTarget(bobHead);
await bob.lookAtTarget(aliceHead);
```

視線追従動作を無効にするには [`unlook`](./unlook) を使用し、マウスカーソルを追従するには [`lookAtCursor`](./lookAtCursor) を使用してください。

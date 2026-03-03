---
title: "視線追従"
sidebar_position: 8
---

# 視線追従（Look At）

VRM キャラクターの視線を制御します。キャラクターはマウスカーソルを追従したり、特定のエンティティを追跡したり、視線追従動作を完全に無効にしたりできます。

## インポート

```typescript
import { Vrm } from "@hmcs/sdk";
```

## カーソル追従

`vrm.lookAtCursor()` はキャラクターの目を画面上のマウスカーソルに追従させます。

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.lookAtCursor();
```

## エンティティを見る

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

## 視線追従の無効化

`vrm.unlook()` は視線追従動作を完全にオフにします。キャラクターの目はデフォルトのアニメーション駆動状態に戻ります。

```typescript
await character.unlook();
```

## 一般的なパターン：状態駆動の視線追従

典型的なパターンは、キャラクターがアイドル状態のときにカーソル追従を有効にし、ドラッグなどのインタラクション中は無効にすることです：

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// カーソル追従を開始
await character.lookAtCursor();

character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    // 短い遅延後にカーソル追従を再開
    //（アニメーショントランジション中のジッターを防止するための遅延）
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    // ドラッグ中は追従を停止
    await character.unlook();
  } else if (e.state === "sitting") {
    // 座っている間もカーソル追従を継続
    await sleep(500);
    await character.lookAtCursor();
  }
});
```

## 例：キャラクター同士の視線

```typescript
const alice = await Vrm.findByName("Alice");
const bob = await Vrm.findByName("Bob");

// 頭のボーンエンティティを取得
const aliceHead = await alice.findBoneEntity("head");
const bobHead = await bob.findBoneEntity("head");

// お互いを見つめ合わせる
await alice.lookAtTarget(bobHead);
await bob.lookAtTarget(aliceHead);
```

## 視線追従の状態

現在の視線追従の状態は `Vrm.findAllDetailed()` から返される `VrmSnapshot` に含まれます：

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  if (s.lookAt === null) {
    console.log(`${s.name}: look-at disabled`);
  } else if (s.lookAt.type === "cursor") {
    console.log(`${s.name}: following cursor`);
  } else if (s.lookAt.type === "target") {
    console.log(`${s.name}: looking at entity ${s.lookAt.entity}`);
  }
}
```

## 型定義

```typescript
type LookAtState =
  | { type: "cursor" }
  | { type: "target"; entity: number };
```

## 次のステップ

- **[スポーンと検索](./spawn-and-find)** -- 視線追従ターゲットのキャラクターとボーンエンティティを検索します。
- **[イベント](./events)** -- 状態変更に反応して視線追従動作を切り替えます。
- **[VRM 概要](./)** -- 完全な API リファレンス表。

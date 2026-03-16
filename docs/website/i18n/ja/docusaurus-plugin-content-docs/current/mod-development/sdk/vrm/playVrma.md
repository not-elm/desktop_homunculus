---
title: "playVrma"
sidebar_position: 16
---

# playVrma

```typescript
import { Vrm, repeat } from "@hmcs/sdk";
```

`vrm.playVrma(options)` はキャラクター上で VRMA アニメーションを開始します。

```typescript
const character = await Vrm.spawn("my-mod:character");

await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## オプション

| オプション              | 型           | デフォルト    | 説明                                                             |
| ------------------- | ------------ | ---------- | ----------------------------------------------------------------------- |
| `asset`             | `string`     | （必須）   | VRMA アニメーションのアセット ID                                          |
| `repeat`            | `VrmaRepeat` | —          | リピートモード：`repeat.forever()`、`repeat.never()`、または `repeat.count(n)` |
| `transitionSecs`    | `number`     | —          | 現在のアニメーションからのクロスフェード時間（秒）                       |
| `waitForCompletion` | `boolean`    | `false`    | `true` の場合、アニメーション完了までブロックします                     |
| `resetSpringBones`  | `boolean`    | `false`    | `true` の場合、バウンスアーティファクトを防ぐためにスプリングボーンの速度をリセットします |

## クロスフェードトランジション

`transitionSecs` を使用して、現在のアニメーションから新しいアニメーションへスムーズにブレンドします。

```typescript
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## 完了待機

`waitForCompletion: true` を設定すると、ワンショットアニメーションが完了するまでブロックします。

```typescript
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
  waitForCompletion: true,
});

// 手を振るアニメーション完了後にこの行が実行されます
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## スプリングボーンリセット

大きく異なるポーズのアニメーション間を切り替える場合、`resetSpringBones: true` を使用してスプリングボーンの速度をリセットします。

```typescript
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
  resetSpringBones: true,
});
```

## ビルトインアニメーション

`@hmcs/assets` MOD は以下のデフォルト VRMA アニメーションを提供します：

| アセット ID            | 説明                       |
| ------------------- | --------------------------------- |
| `vrma:idle-maid`    | 立ちアイドルアニメーション（ループ） |
| `vrma:grabbed`      | つかまれ／持ち上げポーズ（ループ）  |
| `vrma:idle-sitting` | 座りアイドルアニメーション（ループ）  |

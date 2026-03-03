---
title: "アニメーション"
sidebar_position: 4
---

# アニメーション

VRM キャラクター上で VRMA アニメーションを再生します。VRMA は VRM モデル用のアニメーション形式で、ブレンドシェイプ付きのスケルタルアニメーションをサポートしています。ビルトインの `@hmcs/assets` MOD がデフォルトのアニメーションを提供します。

## インポート

```typescript
import { Vrm, repeat } from "@hmcs/sdk";
```

## アニメーションの再生

`vrm.playVrma(options)` はキャラクター上で VRMA アニメーションを開始します。

```typescript
const character = await Vrm.spawn("my-mod:character");

await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

### オプション

| オプション              | 型         | デフォルト    | 説明                                                             |
| ------------------- | ------------ | ---------- | ----------------------------------------------------------------------- |
| `asset`             | `string`     | （必須） | VRMA アニメーションのアセット ID                                          |
| `repeat`            | `VrmaRepeat` | --          | リピートモード：`repeat.forever()`、`repeat.never()`、または `repeat.count(n)` |
| `transitionSecs`    | `number`     | --          | 現在のアニメーションからのブレンドのためのクロスフェード（crossfade）時間（秒）   |
| `waitForCompletion` | `boolean`    | `false`    | `true` の場合、アニメーション完了までブロックします                 |
| `resetSpringBones`  | `boolean`    | `false`    | `true` の場合、バウンスアーティファクトを防ぐためにスプリングボーン（spring bone）の速度をリセットします  |

## リピートモード

`repeat` 名前空間はリピート設定を構築するためのヘルパーを提供します：

```typescript
import { repeat } from "@hmcs/sdk";

// アニメーションを無限ループ
repeat.forever();

// 1回だけ再生して停止
repeat.never();

// 3回再生して停止
repeat.count(3);
```

:::warning
`repeat.count(n)` には正の整数が必要です。0、負の数、非整数を渡すと `RangeError` がスローされます。
:::

## クロスフェードトランジション

`transitionSecs` を使用して、現在のアニメーションから新しいアニメーションへスムーズにブレンドします。指定しない場合、アニメーションは即座に切り替わります。

```typescript
// 0.5秒のスムーズなクロスフェードでアイドルに移行
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// つかまれアニメーションに即座に切り替え
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
});
```

## 完了待機

`waitForCompletion: true` を設定すると、ワンショットアニメーションが完了するまでブロックします。アニメーションの連続実行に便利です。

```typescript
// 手を振るアニメーションを再生し、完了を待機
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
  waitForCompletion: true,
});

// この行は手を振るアニメーション完了後に実行されます
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});
```

## スプリングボーンリセット

大きく異なるポーズのアニメーション間を切り替える場合（例：立ちポーズからつかまれポーズへ）、スプリングボーン（spring bone）の物理演算（髪、衣服）により不要なバウンスが発生することがあります。`resetSpringBones: true` を使用して速度をリセットします。

```typescript
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
  resetSpringBones: true,
});
```

## アニメーション状態のクエリ

### アクティブなアニメーションの一覧

```typescript
const animations = await character.listVrma();
for (const anim of animations) {
  console.log(`${anim.name}: entity=${anim.entity}, playing=${anim.playing}`);
}
```

### 特定のアニメーションを確認

```typescript
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}`);
console.log(`Speed: ${state.speed}x`);
console.log(`Elapsed: ${state.elapsedSecs}s`);
console.log(`Repeat: ${state.repeat}`);
```

### 再生速度の変更

```typescript
// スローモーション
await character.setVrmaSpeed("vrma:idle-maid", 0.5);

// 2倍速
await character.setVrmaSpeed("vrma:idle-maid", 2.0);

// 通常速度
await character.setVrmaSpeed("vrma:idle-maid", 1.0);
```

### アニメーションの停止

```typescript
await character.stopVrma("vrma:idle-maid");
```

## スプリングボーン

VRM モデルは髪、衣服、アクセサリーの物理シミュレーションにスプリングボーン（spring bone）を使用します。スプリングボーンのプロパティを照会・カスタマイズできます。

### すべてのチェーンを照会

```typescript
const { chains } = await character.springBones();
for (const chain of chains) {
  console.log(`Chain ${chain.entity}: ${chain.joints.length} joints`);
  console.log(`  Stiffness: ${chain.props.stiffness}`);
  console.log(`  Drag: ${chain.props.dragForce}`);
}
```

### 物理の変更

```typescript
const { chains } = await character.springBones();
const hairChain = chains[0];

// 髪をより弾力的に
await character.setSpringBone(hairChain.entity, {
  stiffness: 0.5,
  dragForce: 0.2,
});

// 重力方向を変更
await character.setSpringBone(hairChain.entity, {
  gravityPower: 1.0,
  gravityDir: [0, -1, 0],
});
```

## ビルトインアニメーション

`@hmcs/assets` MOD は以下のデフォルト VRMA アニメーションを提供します：

| アセット ID            | 説明                       |
| ------------------- | --------------------------------- |
| `vrma:idle-maid`    | 立ちアイドルアニメーション（ループ） |
| `vrma:grabbed`      | つかまれ／持ち上げポーズ（ループ）  |
| `vrma:idle-sitting` | 座りアイドルアニメーション（ループ）  |

## 型定義

```typescript
interface VrmaPlayRequest {
  asset: string;
  transitionSecs?: number;
  repeat?: VrmaRepeat;
  waitForCompletion?: boolean;
  resetSpringBones?: boolean;
}

interface VrmaRepeat {
  type: "forever" | "never" | "count";
  count?: number;
}

interface VrmaState {
  playing: boolean;
  repeat: string;
  speed: number;
  elapsedSecs: number;
}

interface VrmaInfo {
  entity: number;
  name: string;
  playing: boolean;
}

interface SpringBoneProps {
  stiffness: number;
  dragForce: number;
  gravityPower: number;
  gravityDir: [number, number, number];
  hitRadius: number;
}

interface SpringBoneChain {
  entity: number;
  joints: string[];
  props: SpringBoneProps;
}
```

## 次のステップ

- **[表情](./expressions)** -- アニメーションの上に表情をレイヤリングします。
- **[イベント](./events)** -- `vrma-play` と `vrma-finish` イベントに反応します。
- **[VRM 概要](./)** -- 完全な API リファレンス表。

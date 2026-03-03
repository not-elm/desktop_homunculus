---
title: "Audio（効果音 & BGM）"
sidebar_position: 5
---

# Audio（効果音 & BGM）

効果音（SE）と BGM の再生。音量、フェード、再生制御に対応しています。効果音はワンショット再生で、BGM はトランスポートコントロール付きの連続再生です。

## インポート

```typescript
import { audio } from "@hmcs/sdk";
```

## 効果音

`audio.se.play(asset, options?)` はワンショットの効果音を再生します。サウンドの再生開始後すぐに呼び出しが返ります。

```typescript
// アセット ID で効果音を再生
await audio.se.play("my-mod:click");

// 再生オプション付き
await audio.se.play("my-mod:alert", {
  volume: 0.5,
  speed: 1.2,
  panning: -0.5,  // -1.0 = 左、0.0 = 中央、1.0 = 右
});
```

### `SeOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `volume` | `number` | `1.0` | 音量レベル（0.0--1.0） |
| `speed` | `number` | `1.0` | 再生速度の倍率 |
| `panning` | `number` | `0.0` | ステレオパンニング（-1.0 左 〜 1.0 右） |

## BGM

同時に再生できる BGM トラックは 1 つだけです。新しいトラックを開始すると、現在のトラックが置き換えられます。

### 再生

```typescript
// デフォルトでループ
await audio.bgm.play("my-mod:battle-theme");

// ワンショット再生、フェードイン付き
await audio.bgm.play("my-mod:intro", {
  loop: false,
  volume: 0.6,
  fadeIn: { durationSecs: 3.0, easing: "easeIn" },
});
```

### 停止

```typescript
// 即座に停止
await audio.bgm.stop();

// 2 秒かけてフェードアウト
await audio.bgm.stop({
  fadeOut: { durationSecs: 2.0, easing: "easeOut" },
});
```

### 一時停止と再開

```typescript
await audio.bgm.pause();
await audio.bgm.resume();
```

### 更新

再生中に音量や速度を変更します。`tween` フィールドを使用してトランジションをアニメーションさせることができます。

```typescript
// 1 秒かけて音量を 0.3 にフェード
await audio.bgm.update({
  volume: 0.3,
  tween: { durationSecs: 1.0, easing: "easeInOut" },
});

// 速度を即座に変更
await audio.bgm.update({ speed: 0.8 });
```

### ステータス

```typescript
const status = await audio.bgm.status();
if (status.state === "playing") {
  console.log(`再生中: ${status.asset}、音量: ${status.volume}`);
}
```

## 型

### `BgmPlayOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `loop` | `boolean` | `true` | ループ再生 |
| `volume` | `number` | `1.0` | 音量レベル（0.0--1.0） |
| `speed` | `number` | `1.0` | 再生速度の倍率 |
| `fadeIn` | `FadeTween` | -- | フェードインのトランジション設定 |

### `BgmStopOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `fadeOut` | `FadeTween` | -- | フェードアウトのトランジション設定。省略すると即座に停止します。 |

### `BgmUpdateOptions`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `volume` | `number` | -- | 新しい音量レベル |
| `speed` | `number` | -- | 新しい再生速度 |
| `tween` | `FadeTween` | -- | 変更のトランジション設定 |

### `FadeTween`

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `durationSecs` | `number` | -- | 秒単位の持続時間 |
| `easing` | `string` | `"linear"` | `"linear"`、`"easeIn"`、`"easeOut"`、`"easeInOut"` のいずれか |

### `BgmStatus`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `asset` | `string \| null` | 現在のアセット ID。停止中の場合は `null` |
| `state` | `string` | `"playing"`、`"paused"`、`"stopped"` のいずれか |
| `loop` | `boolean` | ループが有効かどうか |
| `volume` | `number` | 現在の音量レベル |
| `speed` | `number` | 現在の再生速度 |

## 次のステップ

- **[Effects](./effects)** -- 画面上にビジュアルスタンプエフェクトを表示
- **[Signals](./signals)** -- リアルタイム同期のためのクロスプロセス pub/sub メッセージング

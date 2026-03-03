---
title: "effects"
sidebar_position: 8
---

# effects

画面上にビジュアルスタンプエフェクトを表示します -- 指定された位置に、サイズ、不透明度、表示時間を設定可能な画像を表示します。

## インポート

```typescript
import { effects } from "@hmcs/sdk";
```

## スタンプエフェクト

`effects.stamp(asset, options?)` は、画像アセットをスクリーン上の一時的なオーバーレイとして表示します。

```typescript
// 最小構成 -- デフォルトの位置とサイズで表示
await effects.stamp("my-mod:thumbs-up");

// フルオプション
await effects.stamp("my-mod:heart", {
  x: 100,
  y: 200,
  width: 80,
  height: 80,
  alpha: 0.9,
  duration: 1.5,
});
```

### パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `asset` | `string` | スタンプ画像のアセット ID（例: `"my-mod:heart"`） |
| `options` | `StampOptions` | オプションの外観設定 |

## 型

### `StampOptions`

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `x` | `number` | スクリーン上の X 位置（ピクセル） |
| `y` | `number` | スクリーン上の Y 位置（ピクセル） |
| `width` | `number` | 幅（ピクセル） |
| `height` | `number` | 高さ（ピクセル） |
| `alpha` | `number` | 不透明度（0--1） |
| `duration` | `number` | スタンプが消えるまでの秒数 |

## 次のステップ

- **[Audio](./audio)** -- 効果音と BGM の再生
- **[Signals](./signals)** -- クロスプロセス pub/sub メッセージング

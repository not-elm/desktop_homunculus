---
sidebar_position: 100
---

# 型定義

## SeOptions

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `volume` | `number` | `1.0` | 音量レベル（0.0--1.0） |
| `speed` | `number` | `1.0` | 再生速度の倍率 |
| `panning` | `number` | `0.0` | ステレオパンニング（-1.0 左 〜 1.0 右） |

## BgmPlayOptions

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `loop` | `boolean` | `true` | ループ再生 |
| `volume` | `number` | `1.0` | 音量レベル（0.0--1.0） |
| `speed` | `number` | `1.0` | 再生速度の倍率 |
| `fadeIn` | `FadeTween` | -- | フェードインのトランジション設定 |

## BgmStopOptions

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `fadeOut` | `FadeTween` | -- | フェードアウトのトランジション設定。省略すると即座に停止します。 |

## BgmUpdateOptions

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `volume` | `number` | -- | 新しい音量レベル |
| `speed` | `number` | -- | 新しい再生速度 |
| `tween` | `FadeTween` | -- | 変更のトランジション設定 |

## FadeTween

| フィールド | 型 | デフォルト | 説明 |
|-------|------|---------|-------------|
| `durationSecs` | `number` | -- | 秒単位の持続時間 |
| `easing` | `string` | `"linear"` | `"linear"`、`"easeIn"`、`"easeOut"`、`"easeInOut"` のいずれか |

## BgmStatus

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `asset` | `string \| null` | 現在のアセット ID。停止中の場合は `null` |
| `state` | `string` | `"playing"`、`"paused"`、`"stopped"` のいずれか |
| `loop` | `boolean` | ループが有効かどうか |
| `volume` | `number` | 現在の音量レベル |
| `speed` | `number` | 現在の再生速度 |

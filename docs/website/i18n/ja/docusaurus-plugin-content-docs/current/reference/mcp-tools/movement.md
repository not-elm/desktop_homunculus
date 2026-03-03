---
title: "移動"
sidebar_position: 3
---

# 移動

移動ツールはアクティブキャラクターのテレポートまたはトゥイーントランスフォームを行います。

すべての移動ツールはアクティブキャラクターを対象にします。複数のキャラクターを操作する場合は、先に `select_character` を使用してください。

#### `move_character`

アクティブキャラクターをビューポート位置に瞬時にテレポートさせます。`(0, 0)` はプライマリモニターの左上隅です。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `x` | `number` | **必須** | ビューポート X 座標（ピクセル） |
| `y` | `number` | **必須** | ビューポート Y 座標（ピクセル） |

---

#### `tween_position`

アクティブキャラクターの位置をワールド空間のターゲットまでスムーズにアニメーションします。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `targetX` | `number` | **必須** | ターゲット X 座標（ワールド空間） |
| `targetY` | `number` | **必須** | ターゲット Y 座標（ワールド空間） |
| `targetZ` | `number` | **必須** | ターゲット Z 座標（ワールド空間） |
| `durationMs` | `number` | **必須** | アニメーション時間（ミリ秒） |
| `easing` | `string` | `"linear"` | イージング関数（[イージング関数](#イージング関数)を参照） |
| `wait` | `boolean` | `false` | アニメーション完了を待ってから返す |

---

#### `tween_rotation`

アクティブキャラクターの回転をターゲットクォータニオンまでスムーズにアニメーションします。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `targetX` | `number` | **必須** | ターゲットクォータニオン X |
| `targetY` | `number` | **必須** | ターゲットクォータニオン Y |
| `targetZ` | `number` | **必須** | ターゲットクォータニオン Z |
| `targetW` | `number` | **必須** | ターゲットクォータニオン W |
| `durationMs` | `number` | **必須** | アニメーション時間（ミリ秒） |
| `easing` | `string` | `"linear"` | イージング関数（[イージング関数](#イージング関数)を参照） |
| `wait` | `boolean` | `false` | アニメーション完了を待ってから返す |

**例 -- 1秒で Y 軸 180 度回転：**

```json
{
  "targetX": 0,
  "targetY": 1,
  "targetZ": 0,
  "targetW": 0,
  "durationMs": 1000,
  "easing": "cubicInOut"
}
```

---

#### `tween_scale`

アクティブキャラクターのスケールをスムーズにアニメーションします。各軸で `1.0` が通常サイズです。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `targetX` | `number >= 0` | **必須** | ターゲット X スケール係数 |
| `targetY` | `number >= 0` | **必須** | ターゲット Y スケール係数 |
| `targetZ` | `number >= 0` | **必須** | ターゲット Z スケール係数 |
| `durationMs` | `number` | **必須** | アニメーション時間（ミリ秒） |
| `easing` | `string` | `"linear"` | イージング関数（[イージング関数](#イージング関数)を参照） |
| `wait` | `boolean` | `false` | アニメーション完了を待ってから返す |

---

#### イージング関数

3つのトゥイーンツール（`tween_position`、`tween_rotation`、`tween_scale`）はすべて同じイージング値を受け付けます：

`linear`、`quadraticIn`、`quadraticOut`、`quadraticInOut`、`cubicIn`、`cubicOut`、`cubicInOut`、`quarticIn`、`quarticOut`、`quarticInOut`、`quinticIn`、`quinticOut`、`quinticInOut`、`sineIn`、`sineOut`、`sineInOut`、`circularIn`、`circularOut`、`circularInOut`、`exponentialIn`、`exponentialOut`、`exponentialInOut`、`elasticIn`、`elasticOut`、`elasticInOut`、`backIn`、`backOut`、`backInOut`、`bounceIn`、`bounceOut`、`bounceInOut`、`smoothStepIn`、`smoothStepOut`、`smoothStep`、`smootherStepIn`、`smootherStepOut`、`smootherStep`

---
title: "表情"
sidebar_position: 2
---

# 表情

表情ツールは顔のウェイト、アニメーション再生、視線追従動作を制御します。

#### `set_expression`

アクティブキャラクターの表情ウェイトを設定します。ウェイトの範囲は `0.0--1.0` です。

一般的な表情名：`happy`、`sad`、`angry`、`surprised`、`relaxed`、`neutral`、`aa`、`ih`、`ou`、`ee`、`oh`、`blink`。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `expressions` | `Record<string, number>` | -- | 表情名からウェイトへのマップ。`mode` が `"clear"` でない限り必須。 |
| `mode` | `"set" \| "modify" \| "clear"` | `"modify"` | `"modify"` は指定した表情のみ更新、`"set"` はすべて置換、`"clear"` はアニメーション制御状態にリセット。 |

**例 -- やわらかい笑顔：**

```json
{
  "expressions": { "happy": 0.8, "relaxed": 0.3 },
  "mode": "modify"
}
```

---

#### `play_animation`

アクティブキャラクターで VRMA アニメーションを再生します。利用可能な VRMA アセット ID を確認するには `homunculus://assets` リソースを使用してください。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `asset` | `string` | **必須** | VRMA アニメーションアセット ID（例：`vrma:idle-maid`） |
| `repeat` | `"never" \| "forever" \| string` | `"never"` | `"never"` は1回再生、`"forever"` はループ、数字の文字列（例：`"3"`）で N 回繰り返し |
| `transition_secs` | `number` | `0.3` | クロスフェードトランジション時間（秒） |
| `wait` | `boolean` | `false` | アニメーション完了を待ってから返す |
| `reset_spring_bones` | `boolean` | `false` | トランジション時にスプリングボーン（SpringBone）の物理をリセットしてバウンスを防止 |

**例 -- アイドルアニメーションをループ：**

```json
{
  "asset": "vrma:idle-maid",
  "repeat": "forever",
  "transition_secs": 0.5
}
```

---

#### `set_look_at`

アクティブキャラクターの視線方向を制御します。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `mode` | `"cursor" \| "none"` | **必須** | `"cursor"` はマウスポインタを追従、`"none"` は視線追従を無効化 |

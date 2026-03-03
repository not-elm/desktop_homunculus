---
title: "プロンプト"
sidebar_position: 8
---

# プロンプト

プロンプトは AI を特定の MCP ワークフローに誘導するパラメータ化されたテンプレートです。

### `developer-assistant`

開発イベントに対する適切なキャラクターリアクションを生成します。

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `event` | `string` | 開発イベント：`build-success`、`build-failure`、`test-pass`、`test-fail`、`git-push`、`git-commit`、`deploy` |

このプロンプトは、イベントの結果に基づいて適切なプリセットで `play_reaction` を呼び出すよう AI に指示します（例：`build-success` には `success`、`test-fail` には `error`）。

---

### `character-interaction`

デスクトップキャラクターとの自然なインタラクションを行います。

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `message` | `string` | キャラクターに言う、またはキャラクターと行うこと |
| `mood` | `string` | 希望するムード：`happy`、`playful`、`serious`、`encouraging` |

このプロンプトは `get_character_snapshot`、`play_reaction`、およびオプションで `speak_message` を呼び出すよう AI に指示します。

---

### `mod-command-helper`

MOD コマンドを探索し実行します。

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `mod_name` | `string` | 探索する MOD 名 |

このプロンプトは `homunculus://mods` を読み取り、`execute_command` の呼び出し例とともに各コマンドを説明するよう AI に指示します。

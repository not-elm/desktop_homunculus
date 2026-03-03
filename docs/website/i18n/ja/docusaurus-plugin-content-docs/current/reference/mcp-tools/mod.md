---
title: "MOD"
sidebar_position: 6
---

# MOD

MOD 統合ツールはパッケージ化された bin コマンドを実行します。

#### `execute_command`

MOD の bin コマンドを実行します。MOD ごとに利用可能なコマンドを確認するには `homunculus://mods` リソースを使用してください。

| パラメータ | 型 | デフォルト | 説明 |
|-----------|------|---------|-------------|
| `command` | `string` | **必須** | 実行するコマンド名 |
| `args` | `string[]` | -- | コマンド引数 |
| `stdin` | `string` | -- | 標準入力（通常は JSON） |
| `timeoutMs` | `number` | `30000` | タイムアウト（ミリ秒、範囲：1000--300000） |

`stdout`、`stderr`、`exitCode`、`timedOut` を返します。`exitCode !== 0` の場合、ツール結果の `isError` が設定されます。

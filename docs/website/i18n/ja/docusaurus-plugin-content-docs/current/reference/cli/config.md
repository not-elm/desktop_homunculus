---
title: "hmcs config"
sidebar_position: 3
---

# hmcs config

Desktop Homunculus のアプリ設定を管理します。

## クイック例

```shell
hmcs config list
hmcs config get port
hmcs config set port 3200
hmcs config set mods_dir /Users/me/.homunculus/mods
```

## list

### 構文

```shell
hmcs config list
```

### 引数

このサブコマンドには引数はありません。

### 例

成功：

```text
KEY      VALUE
mods_dir /Users/me/.homunculus/mods
port     3100
```

失敗例：

```text
[stderr]
...failed to parse ~/.homunculus/config.toml...
```

### 動作

- `~/.homunculus/config.toml` から設定を読み込みます。
- KEY と VALUE のカラムをキーでソートしたテーブルを出力します。
- 設定ファイルが存在しない場合はデフォルト値が使用されます。

### 関連

- [`hmcs config get`](#get)
- [`hmcs config set`](#set)

## get

### 構文

```shell
hmcs config get <key>
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `key` | はい | 読み取る設定キー。 |

### 例

成功：

```shell
hmcs config get port
```

```text
3100
```

失敗例：

```shell
hmcs config get foo
```

```text
[stderr]
error: unknown config key 'foo'. ...valid keys: mods_dir, port
```

### 動作

- 現在のキーは `mods_dir` と `port` です。
- 不明なキーの場合はゼロ以外で終了します。

### 関連

- [`hmcs config list`](#list)
- [`hmcs config set`](#set)

## set

### 構文

```shell
hmcs config set <key> <value>
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `key` | はい | 書き込む設定キー（`mods_dir` または `port`）。 |
| `value` | はい | 新しい値。可能な場合は TOML リテラルとしてパースされます。 |

### 例

成功：

```shell
hmcs config set port 3200
hmcs config set mods_dir /Users/me/.homunculus/mods
```

失敗例（不明なキー）：

```shell
hmcs config set foo bar
```

```text
[stderr]
error: unknown config key 'foo'. ...valid keys: mods_dir, port
```

失敗例（無効な型）：

```shell
hmcs config set port not_a_number
```

```text
[stderr]
error: invalid value for 'port': ...
```

### 動作

- 現在の設定を読み込み、1つのキー変更を適用してから書き戻します。
- 値のパース順序：
  1. TOML リテラルとしてパース（数値、ブール値、クォートされた文字列用）。
  2. パースに失敗した場合、値をプレーン文字列として扱います。
- 検証に失敗した場合はゼロ以外で終了します。

### 関連

- [`hmcs config get`](#get)
- [`hmcs mod`](./mod)

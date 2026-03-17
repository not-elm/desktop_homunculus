---
title: "hmcs prefs"
sidebar_position: 2
---

# hmcs prefs

SQLite に保存されたキーバリュープリファレンスを管理します。

## クイックスタート

```shell
hmcs prefs list
hmcs prefs get theme
hmcs prefs set theme dark
hmcs prefs set shadow_panel::alpha 0.5
hmcs prefs delete theme
```

## list

### 構文

```shell
hmcs prefs list
```

### 引数

このサブコマンドには引数はありません。

### 例

成功：

```text
theme
shadow_panel::alpha
persona::elmer:vrm
```

保存されたプリファレンスがない場合：

```text
No preferences found.
```

失敗例：

```text
[stderr]
...database error...
```

### 動作

- キーのみを1行に1つずつ出力します。
- `~/.homunculus/preferences.db` から読み取ります。

### 関連

- [`hmcs prefs get`](#get)

## get

### 構文

```shell
hmcs prefs get <key>
```

### 引数

| 名前  | 必須 | 説明                         |
| ----- | ---- | ---------------------------- |
| `key` | はい | 読み込むプリファレンスキー。 |

### 例

成功：

```shell
hmcs prefs get theme
```

```text
dark (string)
```

JSON 値での成功：

```shell
hmcs prefs get profile
```

```text
{
  "voice": "ja",
  "speed": 1.1
} (json)
```

失敗例：

```shell
hmcs prefs get missing_key
```

```text
[stderr]
key not found: missing_key
```

### 動作

- `value (type)` を出力します。`type` は `null`、`bool`、`number`、`string`、`json` のいずれかです。
- JSON 値は整形して出力します。
- キーが存在しない場合はゼロ以外で終了します。

### 関連

- [`hmcs prefs set`](#set)
- [`hmcs prefs delete`](#delete)

## set

### 構文

```shell
hmcs prefs set <key> <value>
```

### 引数

| 名前    | 必須 | 説明                               |
| ------- | ---- | ---------------------------------- |
| `key`   | はい | 書き込むプリファレンスキー。       |
| `value` | はい | 型推論によりパースされる値文字列。 |

### 例

成功：

```shell
hmcs prefs set theme dark
hmcs prefs set ui_scale 1.25
hmcs prefs set enabled true
hmcs prefs set profile '{"voice":"ja","speed":1.1}'
```

失敗例：

```text
[stderr]
...database error...
```

### 動作

- `~/.homunculus/preferences.db` に書き込みます。
- 以下の順序で型を推論します：
  1. `null`
  2. `bool`
  3. `number`
  4. JSON オブジェクトまたは配列
  5. `string`
- 成功時は出力なしです。

### 関連

- [`hmcs prefs get`](#get)
- [`hmcs prefs list`](#list)

## delete

### 構文

```shell
hmcs prefs delete <key>
```

### 引数

| 名前  | 必須 | 説明                         |
| ----- | ---- | ---------------------------- |
| `key` | はい | 削除するプリファレンスキー。 |

### 例

成功：

```shell
hmcs prefs delete theme
```

存在しないキーも成功として扱われます：

```shell
hmcs prefs delete missing_key
```

失敗例：

```text
[stderr]
...database error...
```

### 動作

- キーが存在する場合は削除します。
- キーが存在しなくても成功を返します。
- 成功時は出力なしです。

### 関連

- [`hmcs prefs list`](#list)
- [`hmcs config`](./config)

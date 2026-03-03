---
title: "hmcs mod"
sidebar_position: 4
---

# hmcs mod

MOD パッケージの一覧表示、インストール、アンインストールを行います。

## クイック例

```shell
hmcs mod list
hmcs mod install @hmcs/assets @hmcs/elmer
hmcs mod uninstall @hmcs/assets
```

## list

### 構文

```shell
hmcs mod list
```

### 引数

このサブコマンドには引数はありません。

### 例

成功：

```text
 NAME           VERSION  DESCRIPTION
 @hmcs/elmer    1.0.0    Default character model
 @hmcs/menu     1.0.0    Context menu
```

インストール済み MOD がない場合：

```text
（出力なし）
```

失敗例：

```text
[stderr]
...pnpm ls failed...
```

### 動作

- 設定された mods ディレクトリからインストール済み MOD のメタデータを一覧表示します。
- 内部的に `pnpm -C <mods_dir> ls --parseable -P --depth 0` を使用します。

### 関連

- [`hmcs mod install`](#install)
- [`hmcs mod uninstall`](#uninstall)

## install

### 構文

```shell
hmcs mod install <package>...
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `package` | はい | 1つ以上のパッケージ指定子（例：`@hmcs/elmer` や `pkg@version`）。 |

### 例

成功：

```shell
hmcs mod install @hmcs/assets @hmcs/elmer
```

失敗例（無効なパッケージ名）：

```shell
hmcs mod install 'foo;rm -rf /'
```

```text
[stderr]
invalid package name: contains forbidden characters: foo;rm -rf /
```

失敗例（pnpm add 失敗）：

```text
[stderr]
pnpm add failed with status: ...
```

### 動作

- `pnpm` を呼び出す前にパッケージ名を検証します。
- 設定された `mods_dir` にインストールします。
- 検証またはインストール失敗時にゼロ以外で終了します。

### 関連

- [`hmcs mod list`](#list)
- [`hmcs mod uninstall`](#uninstall)

## uninstall

### 構文

```shell
hmcs mod uninstall <package>...
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `package` | はい | 1つ以上のインストール済みパッケージ名。 |

### 例

成功：

```shell
hmcs mod uninstall @hmcs/assets @hmcs/elmer
```

失敗例（無効なパッケージ名）：

```shell
hmcs mod uninstall '../etc/passwd'
```

```text
[stderr]
invalid package name: contains path traversal: ../etc/passwd
```

失敗例（pnpm remove 失敗）：

```text
[stderr]
pnpm remove failed with status: ...
```

### 動作

- `pnpm` を呼び出す前にパッケージ名を検証します。
- 設定された `mods_dir` からパッケージを削除します。
- 検証またはアンインストール失敗時にゼロ以外で終了します。

### 関連

- [`hmcs mod list`](#list)
- [`hmcs config`](./config)

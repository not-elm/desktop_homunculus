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

## path

### 構文

```shell
hmcs mod path [mods_dir_path]
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `mods_dir_path` | いいえ | 新しい mods ディレクトリパス。省略時は現在のパスを表示します。 |

### 例

現在の mods ディレクトリを表示：

```shell
hmcs mod path
```

```text
/Users/alice/.homunculus/mods
```

mods ディレクトリを変更：

```shell
hmcs mod path ~/custom-mods
```

```text
mods_dir updated to: /Users/alice/custom-mods
```

失敗例（ディレクトリを作成できない場合）：

```text
[stderr]
failed to create directory "/readonly/path": Permission denied
```

### 動作

- 引数なしの場合、`~/.homunculus/config.toml` から現在の `mods_dir` を表示します。
- パス引数がある場合、パスを解決し（`~` や相対パスを展開）、ディレクトリが存在しなければ作成し、更新されたパスを `config.toml` に保存します。
- ディレクトリの作成や設定の保存に失敗した場合、ゼロ以外で終了します。

### 関連

- [`hmcs mod list`](#list)
- [`hmcs config`](./config)

## update

### 構文

```shell
hmcs mod update [mod_patterns...] [--latest|-L]
```

### 引数

| 名前 | 必須 | 説明 |
|---|---|---|
| `mod_patterns` | いいえ | 更新対象の MOD 名パターン。省略時はすべてのインストール済み MOD を更新します。 |
| `--latest`, `-L` | いいえ | MOD を最新バージョンに更新します。 |

### 例

すべてのインストール済み MOD を更新：

```shell
hmcs mod update
```

特定の MOD を更新：

```shell
hmcs mod update @hmcs/elmer @hmcs/assets
```

すべての MOD を最新バージョンに更新：

```shell
hmcs mod update --latest
```

特定の MOD を最新バージョンに更新：

```shell
hmcs mod update @hmcs/elmer -L
```

失敗例（pnpm update 失敗）：

```text
[stderr]
pnpm update failed with status: ...
```

### 動作

- 設定された `mods_dir` 内で `pnpm update` を内部的に使用します。
- `mod_patterns` がない場合、すべてのインストール済み MOD を更新します。
- `--latest` / `-L` を指定すると、`pnpm update` に `--latest` を渡して最新バージョンをインストールします。
- 更新失敗時にゼロ以外で終了します。

### 関連

- [`hmcs mod list`](#list)
- [`hmcs mod install`](#install)

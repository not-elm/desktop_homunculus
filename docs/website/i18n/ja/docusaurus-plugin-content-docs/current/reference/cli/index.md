---
title: "CLI リファレンス"
sidebar_position: 1
---

# CLI リファレンス

`hmcs` は Desktop Homunculus のコマンドラインインターフェースです。

## クイックスタート

```shell
hmcs --help
hmcs prefs --help
hmcs config --help
hmcs mod --help
```

## コマンドマップ

| コマンド           | 用途                                               |
| ----------------- | ----------------------------------------------------- |
| `hmcs prefs ...`  | `preferences.db` のプリファレンス値を読み書きします。 |
| `hmcs config ...` | `config.toml` のアプリ設定値を読み書きします。    |
| `hmcs mod ...`    | MOD パッケージの一覧表示、インストール、アンインストールを行います。            |

## 出力と終了コード

- 成功したコマンドは `0` で終了します。
- 失敗したコマンドはゼロ以外で終了します。
- コマンド出力は標準出力に書き込まれます。
- エラーは標準エラー出力に書き込まれます。

## データパス

| データ           | パス                           |
| -------------- | ------------------------------ |
| アプリ設定     | `~/.homunculus/config.toml`    |
| プリファレンス DB | `~/.homunculus/preferences.db` |

## サブコマンド

- [hmcs prefs](./prefs)
- [hmcs config](./config)
- [hmcs mod](./mod)

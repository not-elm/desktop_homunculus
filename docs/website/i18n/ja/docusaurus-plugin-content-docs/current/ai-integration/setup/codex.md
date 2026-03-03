---
title: "Codex"
sidebar_position: 3
---

# Codex

Codex をセットアップして、Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること
- Node.js 22 以上

## 設定

`codex mcp add` で MCP サーバーを登録します：

```bash
codex mcp add homunculus -- npx -y @hmcs/mcp-server@latest
```

登録を確認するには：

```bash
codex mcp list
```

## 確認

Codex に以下のように尋ねてください：

> 「デスクトップに現在ロードされているキャラクターは何ですか？」

接続が正常に動作していれば、Codex は `homunculus://characters` リソースを読み取り、ロードされているキャラクターを説明します。

## カスタムポート

Desktop Homunculus がデフォルト以外のポートで動作している場合は、MCP サーバー登録時に `HOMUNCULUS_HOST` 環境変数を設定してください：

```bash
codex mcp add homunculus --env HOMUNCULUS_HOST=localhost:4000 -- npx -y @hmcs/mcp-server@latest
```

`homunculus` が既に登録されている場合は、先に削除してから再登録してください：

```bash
codex mcp remove homunculus
```

デフォルト値は `localhost:3100` です。ポートは `~/.homunculus/config.toml` で変更できます。

## 次のステップ

- [MCP リファレンス](/docs/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

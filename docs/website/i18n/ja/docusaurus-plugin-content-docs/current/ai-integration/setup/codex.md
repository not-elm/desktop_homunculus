---
title: "Codex"
sidebar_position: 3
---

# Codex

Codex をセットアップして、Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること

## 設定

`codex mcp add` で MCP サーバーを登録します：

```bash
codex mcp add --transport http homunculus http://localhost:3100/mcp
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

Desktop Homunculus がデフォルト以外のポートで動作している場合（`~/.homunculus/config.toml` で変更）、URL を適宜更新してください：

```bash
codex mcp remove homunculus
codex mcp add --transport http homunculus http://localhost:4000/mcp
```

## 次のステップ

- [MCP リファレンス](/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

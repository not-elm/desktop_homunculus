---
title: "Claude Desktop"
sidebar_position: 1
---

# Claude Desktop

Claude Desktop をセットアップして、Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること

## 設定

Claude Desktop の設定ファイルに以下を追加してください：

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "streamable-http",
      "url": "http://localhost:3100/mcp"
    }
  }
}
```

## 再起動

設定ファイルを保存した後、Claude Desktop を再起動してください。

## 確認

Claude に以下のように尋ねてください：

> 「デスクトップに現在ロードされているキャラクターは何ですか？」

接続が正常に動作していれば、Claude は `homunculus://characters` リソースを読み取り、ロードされているキャラクターを説明します。

## カスタムポート

Desktop Homunculus がデフォルト以外のポートで動作している場合（`~/.homunculus/config.toml` で変更）、URL を適宜更新してください：

```json
{
  "mcpServers": {
    "homunculus": {
      "type": "streamable-http",
      "url": "http://localhost:4000/mcp"
    }
  }
}
```

## 次のステップ

- [MCP リファレンス](/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

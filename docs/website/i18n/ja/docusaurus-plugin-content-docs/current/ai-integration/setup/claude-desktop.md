---
title: "Claude Desktop"
sidebar_position: 1
---

# Claude Desktop

Claude Desktop をセットアップして、Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること
- Node.js 22 以上

## 設定

Claude Desktop の設定ファイルに以下を追加してください：

- **macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
- **Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@latest"]
    }
  }
}
```

## 再起動

設定ファイルを保存した後、Claude Desktop を再起動してください。MCP サーバーは Claude Desktop の起動時に自動的に開始されます。

## 確認

Claude に以下のように尋ねてください：

> 「デスクトップに現在ロードされているキャラクターは何ですか？」

接続が正常に動作していれば、Claude は `homunculus://characters` リソースを読み取り、ロードされているキャラクターを説明します。

## カスタムポート

Desktop Homunculus がデフォルト以外のポートで動作している場合は、`HOMUNCULUS_HOST` 環境変数を設定してください：

```json
{
  "mcpServers": {
    "homunculus": {
      "command": "npx",
      "args": ["-y", "@hmcs/mcp-server@latest"],
      "env": {
        "HOMUNCULUS_HOST": "localhost:4000"
      }
    }
  }
}
```

デフォルト値は `localhost:3100` です。ポートは `~/.homunculus/config.toml` で変更できます。

## 次のステップ

- [MCP リファレンス](/docs/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

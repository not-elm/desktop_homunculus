---
title: "Claude Code"
sidebar_position: 2
---

# Claude Code

Claude Code をセットアップして、ターミナルから Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること
- Node.js 22 以上

## 設定

Homunculus MCP サーバーを Claude Code の設定に追加します。

**プロジェクトレベル**（プロジェクトルートの `.mcp.json`）：

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

**グローバル**（`~/.claude/settings.json`）：

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

> 最新の設定オプションについては [Claude Code ドキュメント](https://docs.anthropic.com/en/docs/claude-code)をご覧ください。設定の形式はバージョン間で変更される可能性があります。

## 確認

Claude Code に以下のように尋ねてください：

> 「デスクトップに現在ロードされているキャラクターは何ですか？」

接続が正常に動作していれば、Claude Code は `homunculus://characters` リソースを読み取り、ロードされているキャラクターを説明します。

## スキル

Claude Code の **スキル（Skills）** は MCP ツール呼び出しを連鎖させて複雑なワークフローを実現します。例えば、`tech-lecture` スキルは `open_webview`、`speak_message`、`set_expression` を組み合わせて、キャラクターにスライドとナレーション付きのレクチャーを行わせます。

公式スキルはリポジトリの [`skills/` ディレクトリ](https://github.com/not-elm/desktop_homunculus/tree/main/skills)にあります。インストール方法と完全なカタログについては、そこの README をご覧ください。

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

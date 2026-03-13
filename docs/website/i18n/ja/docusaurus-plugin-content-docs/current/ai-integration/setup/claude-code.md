---
title: "Claude Code"
sidebar_position: 2
---

# Claude Code

Claude Code をセットアップして、ターミナルから Desktop Homunculus のキャラクターを操作できるようにします。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること

## 設定

Homunculus MCP サーバーを Claude Code の設定に追加します。

**プロジェクトレベル**（プロジェクトルートの `.mcp.json`）：

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

**グローバル**（`~/.claude/settings.json`）：

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

> 最新の設定オプションについては [Claude Code ドキュメント](https://docs.anthropic.com/en/docs/claude-code)をご覧ください。設定の形式はバージョン間で変更される可能性があります。

## 確認

Claude Code に以下のように尋ねてください：

> 「デスクトップに現在ロードされているキャラクターは何ですか？」

接続が正常に動作していれば、Claude Code は `homunculus://characters` リソースを読み取り、ロードされているキャラクターを説明します。

## スキル

Claude Code の **スキル（Skills）** は MCP ツール呼び出しを連鎖させて複雑なワークフローを実現します。例えば、`tech-lecture` スキルは `open_webview`、`speak_message`、`set_expression` を組み合わせて、キャラクターにスライドとナレーション付きのレクチャーを行わせます。

公式スキルはリポジトリの [`skills/` ディレクトリ](https://github.com/not-elm/desktop_homunculus/tree/main/skills)にあります。インストール方法と完全なカタログについては、そこの README をご覧ください。

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

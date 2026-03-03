---
title: "その他の MCP クライアント"
sidebar_position: 4
---

# その他の MCP クライアント

MCP 対応のクライアントであれば、どれでも Desktop Homunculus に接続できます。MCP サーバーは **stdio トランスポート**を使用します。クライアントがサーバーをサブプロセスとして起動し、stdin/stdout を通じて通信します。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること
- Node.js 22 以上

## サーバー設定

MCP クライアントを以下の設定でサーバーを起動するように構成してください：

- **コマンド:** `npx`
- **引数:** `["-y", "@hmcs/mcp-server@latest"]`
- **トランスポート:** stdio

## 環境変数

| 変数 | デフォルト | 説明 |
|---|---|---|
| `HOMUNCULUS_HOST` | `localhost:3100` | Desktop Homunculus HTTP API のホストとポート |

Desktop Homunculus がデフォルト以外のポートで動作している場合は `HOMUNCULUS_HOST` を設定してください。

## 確認

接続後、`homunculus://characters` リソースを読み取ってください。キャラクターデータが返されれば、接続は正常に動作しています。

## 次のステップ

- [MCP リファレンス](/docs/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [MCP プロトコル仕様](https://modelcontextprotocol.io) — MCP の公式ドキュメント
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

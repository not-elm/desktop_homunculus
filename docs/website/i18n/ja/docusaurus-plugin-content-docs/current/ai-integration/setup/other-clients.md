---
title: "その他の MCP クライアント"
sidebar_position: 4
---

# その他の MCP クライアント

MCP 対応のクライアントであれば、どれでも Desktop Homunculus に接続できます。MCP サーバーは **Streamable HTTP**を使用します。クライアントがエンジンの HTTP エンドポイントに直接接続します。

## 前提条件

- Desktop Homunculus がインストールされて起動中であること

## サーバー設定

MCP クライアントを以下の設定で接続してください：

- **URL:** `http://localhost:3100/mcp`
- **トランスポート:** Streamable HTTP

別途サーバープロセスのインストールは不要です — MCP サーバーは Desktop Homunculus エンジンに内蔵されています。

## カスタムポート

デフォルトポートは `3100` です。`~/.homunculus/config.toml` で変更できます。クライアント設定の URL を適宜更新してください。

## 確認

接続後、`homunculus://characters` リソースを読み取ってください。キャラクターデータが返されれば、接続は正常に動作しています。

## 次のステップ

- [MCP リファレンス](/reference/mcp-tools) — 利用可能なすべてのツール、リソース、プロンプトを確認
- [MCP プロトコル仕様](https://modelcontextprotocol.io) — MCP の公式ドキュメント
- [トラブルシューティング](../troubleshooting) — よくある問題と解決策

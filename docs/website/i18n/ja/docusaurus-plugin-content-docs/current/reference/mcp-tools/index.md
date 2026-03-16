---
title: "MCP リファレンス"
sidebar_position: 1
slug: /reference/mcp-tools
---

# MCP リファレンス

Desktop Homunculus の MCP サーバーは、Streamable HTTP を介して 18 のツール、4 つのリソース、3 つのプロンプトを公開しています。

このページをマップとして使い、必要なカテゴリページを開いてください。

## MCP サーバー概要

MCP サーバーは Desktop Homunculus エンジンに内蔵されており、`http://localhost:3100/mcp` でアクセスできます。ポートは `~/.homunculus/config.toml` で変更できます。

## カテゴリマップ

| カテゴリ | 内容 | リンク |
|---|---|---|
| キャラクター | キャラクターのスポーン・選択・削除、ペルソナとスナップショットの管理（5ツール） | [キャラクター](./mcp-tools/character) |
| 表情 | 表情、リアクション、アニメーション、視線追従用の4ツール | [表情](./mcp-tools/expression) |
| 移動 | 移動とトゥイーントランスフォーム用の4ツール | [移動](./mcp-tools/movement) |
| オーディオ | スピーチ、効果音、BGM 制御用の3ツール | [オーディオ](./mcp-tools/audio) |
| WebView | WebView のライフサイクルとコンテンツ更新用の3ツール | [WebView](./mcp-tools/webview) |
| MOD | MOD コマンド実行用の1ツール | [MOD](./mcp-tools/mod) |
| リソース | 4つの読み取り専用リソースエンドポイント | [リソース](./mcp-tools/resources) |
| プロンプト | 3つのパラメータ化されたワークフロープロンプト | [プロンプト](./mcp-tools/prompts) |

## ツール別カテゴリクイックリファレンス

| ツール | カテゴリ |
|---|---|
| `get_character_snapshot` | キャラクター |
| `spawn_character` | キャラクター |
| `remove_character` | キャラクター |
| `select_character` | キャラクター |
| `set_persona` | キャラクター |
| `set_expression` | 表情 |
| `play_animation` | 表情 |
| `set_look_at` | 表情 |
| `move_character` | 移動 |
| `tween_position` | 移動 |
| `tween_rotation` | 移動 |
| `tween_scale` | 移動 |
| `play_sound` | オーディオ |
| `control_bgm` | オーディオ |
| `open_webview` | WebView |
| `close_webview` | WebView |
| `navigate_webview` | WebView |
| `execute_command` | MOD |

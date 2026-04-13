---
title: "Desktop Homunculus とは？"
sidebar_position: 1
---

# Desktop Homunculus とは？

Desktop Homunculus は、[Bevy](https://bevyengine.org/) ゲームエンジンで構築されたクロスプラットフォームのデスクトップマスコットアプリケーションです。VRM ベースのキャラクター作成、WebView ベースの UI オーバーレイ、MOD システムによる高い拡張性を備えています。

## 主な機能

- **VRM 3D キャラクターレンダリング** — キャラクターがデスクトップ上に表示され、ワークスペースに自然に溶け込みます
- **Persona システム** — 個別のアイデンティティ（名前、年齢、性別、性格）と永続的な設定を持つ複数のキャラクターを管理できます
- **MOD システム** — MOD をインストール・作成して、キャラクター、アニメーション、効果音、UI パネル、カスタム動作を追加できます。MOD は pnpm で管理される npm パッケージです
- **AI Agent** — キャラクターが Claude や Codex との連携により自律的な AI エージェントとして動作します。音声入力やワークスペース認識にも対応
- **Speech-to-Text** — Whisper ベースの音声認識と Push-to-Talk でハンズフリー操作が可能です
- **MCP による AI 連携** — Model Context Protocol（MCP）サーバーを通じて、AI アシスタント（Claude など）でキャラクターを操作できます
- **WebView ベースの UI オーバーレイ** — 設定パネル、コンテキストメニュー、カスタム UI は Chromium Embedded Framework（CEF）を介してエンジン内でレンダリングされる React アプリです
- **拡張可能な TypeScript SDK** — `@hmcs/sdk` パッケージにより、MOD 開発者が Persona、WebView UI、オーディオ、設定を制御できます

:::info[VRM とは？]
[VRM](https://vrm.dev/) は 3D ヒューマノイドアバターのためのオープンファイルフォーマットです。Desktop Homunculus は VRM 1.0 モデルをキャラクターフォーマットとして使用します。VRM モデルは [VRoid Studio](https://vroid.com/studio) などのツールで作成したり、[VRoid Hub](https://hub.vroid.com/) などのプラットフォームで入手できます。
:::

## 必要な環境

| 要件 | バージョン |
|---|---|
| **OS** | macOS 12+ / Windows 10+ |

:::info[アルファ版について]
Desktop Homunculus は現在 **アルファ版** です。API や MOD の仕様はリリース間で変更される可能性があります。フィードバックやコントリビューションを歓迎しています。
:::

## 次のステップ

- **[インストール](/getting-started/installation)** — アプリをダウンロードして環境をセットアップ
- **[基本コンセプト](/getting-started/concepts)** — Persona、MOD、Asset について理解する
- **[クイックスタート](/getting-started/quick-start)** — 数分で使い始める：設定、キャラクターとのインタラクション、[公式 MOD](/mods/) の探索
- **[MOD 開発](/mod-development)** — TypeScript SDK で独自の MOD を開発
- **[AI 連携](/ai-integration)** — MCP でキャラクターに AI アシスタントを接続、または組み込みの AI Agent を利用

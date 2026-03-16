---
title: "Desktop Homunculus とは？"
sidebar_position: 1
---

# Desktop Homunculus とは？

Desktop Homunculus は、[Bevy](https://bevyengine.org/) ゲームエンジンで構築されたクロスプラットフォームのデスクトップマスコットアプリケーションです。透明ウィンドウ上に VRM 3D キャラクターをデスクトップに直接レンダリングし、設定、メニュー、カスタムインターフェース用の WebView ベースの UI オーバーレイを備えています。MOD システムによる拡張を前提に設計されており、キャラクター、動作、連携機能をカスタマイズできます。

## 主な機能

- **VRM 3D キャラクターレンダリング** — キャラクターは透明ウィンドウ内にレンダリングされ、デスクトップ上でワークスペースに自然に溶け込みます
- **MOD システム** — MOD をインストール・作成して、キャラクター、アニメーション、効果音、UI パネル、カスタム動作を追加できます。MOD は pnpm で管理される npm パッケージです
- **MCP による AI 連携** — Model Context Protocol（MCP）サーバーを通じて、AI アシスタント（Claude など）でキャラクターを操作できます
- **WebView ベースの UI オーバーレイ** — 設定パネル、コンテキストメニュー、カスタム UI は、Chromium Embedded Framework（CEF）を介してエンジン内でレンダリングされる React アプリです
- **拡張可能な TypeScript SDK** — `@hmcs/sdk` パッケージが、キャラクター、アニメーション、オーディオ、プリファレンスなどを操作するための型付き API を MOD 開発者に提供します

## 仕組み

Bevy 上に構築されたエンジンが、VRM キャラクターを含む透明なデスクトップウィンドウをレンダリングします。MOD はエンジンと並行して Node.js 子プロセスとして動作する npm パッケージで、`localhost:3100` の HTTP API を通じて通信します。TypeScript SDK（`@hmcs/sdk`）はこの API をラップし、キャラクターの生成、アニメーションの再生、プリファレンスの管理、WebView の表示などを行う便利な型付き関数を提供します。AI アシスタントは、`localhost:3100/mcp` の内蔵 MCP サーバー（Streamable HTTP）を通じてキャラクターを操作できます。

## 動作要件

| 要件 | バージョン |
|---|---|
| **OS** | macOS 12 以降（Windows サポートは予定） |
| **Node.js** | 22 以上 |
| **pnpm** | 10.x |

:::info[アルファ版について]
Desktop Homunculus は現在 **アルファ版**（v0.1.0-alpha.4）です。API や MOD の仕様はリリース間で変更される可能性があります。フィードバックやコントリビューションを歓迎しています。
:::

## 次のステップ

- **[インストール](/getting-started/installation)** — アプリをダウンロードして MOD 環境をセットアップ
- **[クイックスタート](/getting-started/quick-start)** — 数分で使い始める：設定の構成、キャラクターとのインタラクション、[公式 MOD](/mods/) の探索
- **[MOD 開発](/mod-development)** — TypeScript SDK で独自の MOD を作成
- **[AI 連携](/ai-integration)** — MCP を通じて AI アシスタントをキャラクターに接続

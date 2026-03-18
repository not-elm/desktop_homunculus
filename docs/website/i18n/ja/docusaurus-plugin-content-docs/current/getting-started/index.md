---
title: "Desktop Homunculus とは？"
sidebar_position: 1
---

# Desktop Homunculus とは？

Desktop Homunculus は、[Bevy](https://bevyengine.org/) ゲームエンジンで構築されたクロスプラットフォームのデスクトップマスコットアプリケーションです。 VRMを使ったキャラクター生成、WebView ベースの UI オーバーレイを備えています。また、MODによる高い拡張性を提供します。

## 主な機能

- **VRM 3D キャラクターレンダリング** — キャラクターはデスクトップ上にレンダリングされ、ワークスペースに自然に溶け込みます
- **MOD システム** — MOD をインストール・作成して、キャラクター、アニメーション、効果音、UI パネル、カスタム動作を追加できます。MOD は pnpm で管理される npm パッケージです
- **MCP による AI 連携** — Model Context Protocol（MCP）サーバーを通じて、AI アシスタント（Claude など）でキャラクターを操作できます
- **WebView ベースの UI オーバーレイ** — 設定パネル、コンテキストメニュー、カスタム UI は、Chromium Embedded Framework（CEF）を介してエンジン内でレンダリングされる React アプリです
- **拡張可能な TypeScript SDK** — MOD開発者向けに`@hmcs/sdk` というSDKを提供しています。これはキャラクター、WebviewUI、オーディオ、設定機能の制御などを提供します。

:::info[VRM とは？]
[VRM](https://vrm.dev/) は、3D ヒューマノイドアバター用のオープンファイルフォーマットです。Desktop Homunculus は VRM 1.0 モデルをキャラクターフォーマットとして使用しています。[VRoid Studio](https://vroid.com/studio) などのツールで VRM モデルを作成したり、[VRoid Hub](https://hub.vroid.com/) などのプラットフォームで見つけることができます。
:::

## 動作要件

| 要件        | バージョン                              |
| ----------- | --------------------------------------- |
| **OS**      | macOS 12 以降（Windows サポートは予定） |
| **Node.js** | 22 以上                                 |
| **pnpm**    | 10.x                                    |

:::info[アルファ版について]
Desktop Homunculus は現在 **アルファ版**です。API や MOD の仕様はリリース間で変更される可能性があります。フィードバックやコントリビューションを歓迎しています。
:::

## 次のステップ

- **[インストール](/getting-started/installation)** — アプリをダウンロードして MOD 環境をセットアップ
- **[クイックスタート](/getting-started/quick-start)** — 数分で使い始める：設定の構成、キャラクターとのインタラクション、[公式 MOD](/mods/) の探索
- **[MOD 開発](/mod-development)** — TypeScript SDK で独自の MOD を作成
- **[AI 連携](/ai-integration)** — MCP を通じて AI アシスタントをキャラクターに接続

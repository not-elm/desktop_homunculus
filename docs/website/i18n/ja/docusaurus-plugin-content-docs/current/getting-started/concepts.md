---
title: "基本コンセプト"
sidebar_position: 3
---

# 基本コンセプト

クイックスタートに進む前に、Desktop Homunculus 全体で登場する 3 つの基本概念を紹介します。

## Persona

**Persona** は、デスクトップ上に住むキャラクターです。各 Persona は以下の要素で構成されます：

- **アイデンティティ** — キャラクターを定義する名前、年齢、性別、性格の説明、プロフィールテキスト
- **VRM モデル** — 他のアプリケーションの上に浮かぶ透明ウィンドウでレンダリングされる 3D アバター。組み込みモデルを使用するか、自分の [VRM](https://vrm.dev/) ファイルを使用できます
- **永続設定** — セッション間で保存される外見の調整（ボーンスケール）や各種設定

複数の Persona を作成でき、それぞれ独自の外見と性格を持たせることができます。Persona の **Auto-Spawn** を有効にすると、アプリ起動時に自動的にキャラクターが表示されます。

Persona はシステムトレイからアクセスできる **Persona Management** ダッシュボードで管理します。

## MOD

**MOD** は、Desktop Homunculus に機能を追加するパッケージです。Persona 管理、アニメーション、右クリックメニューなどのコア機能は、インストーラーに同梱される公式 MOD によって提供されます。

公式 MOD は `@hmcs/` プレフィックスを使用します：

| MOD | 機能 |
|---|---|
| `@hmcs/persona` | Persona 管理とデフォルトのキャラクター動作 |
| `@hmcs/assets` | 組み込み VRM モデル、アニメーション、効果音 |
| `@hmcs/menu` | 右クリックコンテキストメニュー |
| `@hmcs/settings` | アプリケーション設定パネル |
| `@hmcs/app-exit` | システムトレイの終了オプション |

テキスト読み上げ（`@hmcs/voicevox`）、AI Agent（`@hmcs/agent`）、音声認識（`@hmcs/stt`）などの追加 MOD は、`hmcs` CLI でいつでもインストールできます。

## Asset

**Asset** は、MOD にバンドルされたファイルです — VRM モデル、アニメーション（VRMA）、効果音、画像、HTML UI など。Asset は **`mod-name:asset-id`** の形式の ID で参照します。

例：
- `vrm:elmer` — 組み込みの VRM キャラクターモデル
- `vrma:idle-maid` — デフォルトの待機アニメーション
- `se:open` — UI オープン時の効果音

`@hmcs/assets` MOD が、Persona システムを動かすデフォルトの Asset セットを提供します。

## 次のステップ

準備はできましたか？[クイックスタート](./quick-start)ガイドに進んで、最初の Persona を作成しましょう。

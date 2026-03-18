---
title: "SDK"
sidebar_position: 4
---

# SDK

`@hmcs/sdk` は、Desktop Homunculus MOD を構築するための公式 TypeScript SDK です。エンジンの HTTP API を型安全なメソッド、リアルタイムイベントストリーミング、キャラクター制御・オーディオ・UI などの高レベル抽象化でラップしています。

モジュールごとの完全な API リファレンスは **[SDK リファレンス](/reference/sdk)** を参照してください。

## クイックスタート

### インストール

```bash
pnpm add @hmcs/sdk
```

SDK は **Node.js 22 以降** が必要です。MOD スクリプトは `tsx` により TypeScript として直接実行されます -- ビルドステップは不要です。

### 最初のスクリプト

MOD のサービススクリプトは、Desktop Homunculus の起動時に自動的に実行されます。MOD のルートに `index.ts` を作成してください：

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

// package.json で宣言されたアセットを使用して VRM キャラクターをスポーン
const character = await Vrm.spawn("my-mod:character");

// 組み込みのアイドルアニメーションをループ再生
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// キャラクターの目がマウスカーソルを追従するように設定
await character.lookAtCursor();

// 状態変化を監視（drag、idle、sitting）
character.events().on("state-change", async (e) => {
  console.log("状態が変わりました:", e.state);
});
```

`package.json` の `"homunculus"` フィールド内で `"service": "index.ts"` を設定し、起動時にどのファイルを実行するかエンジンに伝えてください。

### 主要な概念

#### アセット ID

アセットは `"mod-name:asset-name"` 形式のグローバルに一意な文字列 ID で参照されます。例えば：

- `"my-mod:character"` -- `my-mod` が宣言した VRM モデル
- `"vrma:idle-maid"` -- 組み込みの `@hmcs/assets` MOD からの VRMA アニメーション

アセットは MOD の `package.json` の `homunculus.assets` フィールドで宣言されます。詳細は [アセット ID](./project-setup/asset-ids) を参照してください。

#### HTTP API

SDK はエンジンの HTTP REST API（`localhost:3100` で動作）をラップしています。各 SDK モジュール（`Vrm`、`entities`、`audio` など）は、メソッド呼び出しを HTTP リクエストに変換します。API を直接呼び出す必要があることはほとんどありませんが、高度なユースケースでは `host` モジュールを介して利用できます。

#### イベント駆動パターン

MOD は 2 つのメカニズムを使用してリアルタイムイベントに対応します：

- **VRM イベント** -- ポインタークリック、ドラッグ、状態変化、アニメーションイベント（`vrm.events()` 経由）
- **Signals** -- MOD スクリプトと WebView 間の通信のためのクロスプロセス pub/sub メッセージング

### Commands エントリーポイント

`@hmcs/sdk/commands` は、MOD コマンドスクリプト（`package.json` の `"bin"` で宣言される MOD コマンド）のための **別エントリーポイント** です。stdin のパースと構造化された出力ヘルパーを提供します。完全な API リファレンスは [Commands](/reference/sdk/commands) ページを参照してください。

:::warning
MOD のメインスクリプトやブラウザ側のコードから `@hmcs/sdk/commands` をインポート **しないでください**。`process.stdin` を使用しており、Node.js の MOD コマンドスクリプトでのみ利用可能です。
:::

## 次のステップ

- **[SDK リファレンス](/reference/sdk)** -- 全 18 モジュールの完全なモジュールマップ
- **[MOD クイックスタート](./quick-start)** -- MOD 作成のエンドツーエンドチュートリアル

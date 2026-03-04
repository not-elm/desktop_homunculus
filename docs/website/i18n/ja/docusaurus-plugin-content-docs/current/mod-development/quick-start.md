---
title: "クイックスタート"
sidebar_position: 2
---

# クイックスタート

Desktop Homunculus の最初の MOD をゼロから構築します。このガイドの最後には、VRM キャラクターを生成し、アイドルアニメーションを再生し、ユーザー操作に反応する動作する MOD が完成します。

## 前提条件

始める前に、以下を準備してください：

- **Node.js 22 以降** -- `tsx` による TypeScript サポートに必要
- **pnpm** -- MOD システムが使用するパッケージマネージャー
- **Desktop Homunculus** がマシン上で動作していること
- **`hmcs` CLI** がグローバルにインストールされていること（[インストール](/getting-started/installation)を参照）

:::tip
続行する前に `node -v` と `hmcs --version` でセットアップを確認してください。
:::

## Step 1: プロジェクトの作成

新しいディレクトリを作成し、npm パッケージとして初期化します：

```bash
mkdir my-character
cd my-character
pnpm init
```

SDK をインストールします：

```bash
pnpm add @hmcs/sdk
```

`@hmcs/sdk` はキャラクターの制御、サウンドの再生などの TypeScript API を提供します。MOD が組み込みアニメーション（`vrma:idle-maid` など）を使用する場合、`@hmcs/assets` を別途インストールしてください：`hmcs mod install @hmcs/assets`。

## Step 2: package.json の設定

`package.json` を開き、`homunculus` フィールドと `type` フィールドを追加します。VRM モデルファイルも必要です。プロジェクト内の `assets/` ディレクトリに配置してください。

```json
{
  "name": "my-character",
  "version": "1.0.0",
  "type": "module",
  "dependencies": {
    "@hmcs/sdk": "..."
  },
  "homunculus": {
    "service": "index.ts",
    "assets": {
      "my-character:vrm": {
        "path": "assets/MyModel.vrm",
        "type": "vrm",
        "description": "My custom VRM character"
      }
    }
  }
}
```

## Step 3: サービスの作成

プロジェクトルートに `index.ts` を作成します。このスクリプトは Desktop Homunculus の起動時に自動実行されます。

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

// VRM キャラクターを画面上に生成
const character = await Vrm.spawn("my-character:vrm");

// 組み込みのアイドルアニメーションをループ再生
const animationOptions = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

await character.playVrma({
  asset: "vrma:idle-maid",
  ...animationOptions,
});

// キャラクターがカーソルを追従するように設定
await character.lookAtCursor();

// 状態変化（ドラッグ、アイドル、座り）に反応
character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await character.playVrma({
      asset: "vrma:idle-maid",
      ...animationOptions,
    });
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
    await character.playVrma({
      asset: "vrma:grabbed",
      ...animationOptions,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await character.playVrma({
      asset: "vrma:idle-sitting",
      ...animationOptions,
    });
    await character.lookAtCursor();
  }
});
```

このスクリプトは 3 つのことを行います：

1. `my-character:vrm` として登録された VRM モデルを**生成**
2. 組み込みの `vrma:idle-maid` アニメーションをループで**再生**
3. ユーザーがキャラクターをドラッグ・ドロップした際にアニメーションを切り替えるために状態変化を**監視**

## Step 4: インストールとテスト

`hmcs` CLI を使って MOD をインストールします：

```bash
hmcs mod install /path/to/my-character
```

Desktop Homunculus を再起動します。キャラクターがデスクトップ上に表示されるはずです。ドラッグしてアニメーションが変わることを確認してみてください。

## Step 5: 反復開発

MOD に変更を加えた場合：

1. `hmcs mod install /path/to/my-character` を再度実行してインストール済みのコピーを更新
2. Desktop Homunculus を再起動して変更を反映

## 次のステップ

- **[パッケージ設定](./project-setup/package-json.md)** -- `bin` コマンドやメニューを含む `package.json` の全フィールドについて学ぶ
- **[アセット ID](./project-setup/asset-ids.md)** -- MOD 間でのアセット識別子の仕組みを理解する
- **[MOD とは？](./index.md)** -- MOD が提供できる機能について学ぶ

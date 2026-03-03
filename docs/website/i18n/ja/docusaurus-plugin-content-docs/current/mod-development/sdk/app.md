---
title: "app"
sidebar_position: 15
---

# app

アプリケーションライフサイクル、ヘルスチェック、プラットフォーム情報を提供します。`app` を使用して、エンジンが実行中かどうかの確認、バージョンや機能の照会、アプリケーションのシャットダウンを行えます。

## インポート

```typescript
import { app } from "@hmcs/sdk";
```

## ヘルスチェック

Desktop Homunculus エンジンが到達可能で正常であれば `true` を、そうでなければ `false` を返します。エンジンの起動を待ってから処理を続行する必要があるサービスに便利です。

```typescript
const alive = await app.health();
if (!alive) {
  console.error("Homunculus エンジンが実行されていません");
}
```

**シグネチャ：**

```typescript
app.health(): Promise<boolean>
```

## アプリ情報

実行中のエンジンインスタンスに関するメタデータを 1 回のリクエストで返します -- バージョン文字列、プラットフォームの詳細、コンパイルされた機能、読み込み済みの全 MOD。

```typescript
const info = await app.info();
console.log(`Engine v${info.version} on ${info.platform.os}/${info.platform.arch}`);
console.log(`Features: ${info.features.join(", ")}`);
console.log(`${info.mods.length} 個の MOD が読み込まれています`);

for (const mod of info.mods) {
  console.log(`  ${mod.name}@${mod.version} — ${mod.binCommands.length} コマンド`);
}
```

**シグネチャ：**

```typescript
app.info(): Promise<AppInfo>
```

## 終了

Desktop Homunculus アプリケーションをグレースフルにシャットダウンします。

```typescript
await app.exit();
```

:::warning
`app.exit()` はすべての実行中の MOD を含むアプリケーション全体を終了します。注意して使用してください。
:::

## 型

### AppInfo

```typescript
interface AppInfo {
  /** エンジンのバージョン文字列（例: "0.1.0-alpha.4"）。 */
  version: string;
  /** プラットフォーム情報。 */
  platform: PlatformInfo;
  /** このビルドで利用可能なエンジンレベルの機能。 */
  features: string[];
  /** メタデータ付きの全読み込み済み MOD。 */
  mods: InfoMod[];
}
```

### PlatformInfo

```typescript
interface PlatformInfo {
  /** オペレーティングシステム（例: "macos", "windows", "linux"）。 */
  os: string;
  /** CPU アーキテクチャ（例: "aarch64", "x86_64"）。 */
  arch: string;
}
```

### InfoMod

```typescript
interface InfoMod {
  /** MOD パッケージ名。 */
  name: string;
  /** MOD パッケージバージョン。 */
  version: string;
  /** 人間が読める説明。 */
  description: string | null;
  /** 作者。 */
  author: string | null;
  /** ライセンス識別子。 */
  license: string | null;
  /** MOD にメインプロセスが実行中かどうか。 */
  hasMain: boolean;
  /** 利用可能な bin コマンド名。 */
  binCommands: string[];
  /** 登録済みアセット ID。 */
  assetIds: string[];
}
```

## 次のステップ

- **[SDK 概要](./)** -- 完全なモジュールマップとクイック例。

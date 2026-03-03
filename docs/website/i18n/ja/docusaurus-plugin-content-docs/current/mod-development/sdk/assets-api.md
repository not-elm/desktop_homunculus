---
title: "assets"
sidebar_position: 13
---

# assets

アセットレジストリへのクエリ -- アセットの種類や MOD でフィルタリングして一覧を取得します。アセットは各 MOD の `package.json` で宣言され、`"mod-name:asset-name"` 形式のグローバルに一意な ID で参照されます。

## インポート

```typescript
import { assets } from "@hmcs/sdk";
```

## アセット一覧

`assets.list(filter?)` は登録されたすべてのアセットを返します。オプションで種類や MOD 名によるフィルタリングが可能です。

```typescript
// すべてのアセットを取得
const all = await assets.list();

// VRM モデルのみ取得
const vrms = await assets.list({ type: "vrm" });

// 特定の MOD のアセットを取得
const elmerAssets = await assets.list({ mod: "elmer" });

// フィルタを組み合わせる
const sounds = await assets.list({ type: "sound", mod: "my-mod" });
```

## アセットの種類

| 種類 | 説明 |
|------|-------------|
| `vrm` | VRM 3D キャラクターモデル |
| `vrma` | VRM キャラクター用の VRMA アニメーションファイル |
| `sound` | 音声ファイル（効果音、BGM） |
| `image` | 画像ファイル（PNG、JPG など） |
| `html` | WebView コンテンツ用の HTML ファイル |

## 型定義

### AssetType

```typescript
type AssetType = "vrm" | "vrma" | "sound" | "image" | "html";
```

### AssetInfo

```typescript
interface AssetInfo {
  /** グローバルに一意なアセット ID（例："elmer:character"） */
  id: string;
  /** アセットの種類 */
  type: AssetType;
  /** このアセットを提供する MOD */
  mod: string;
  /** アセットの説明（オプション） */
  description?: string;
}
```

### AssetFilter

```typescript
interface AssetFilter {
  /** アセットの種類でフィルタ */
  type?: AssetType;
  /** MOD 名でフィルタ */
  mod?: string;
}
```

## 使用例

### 利用可能なキャラクターをすべて検索

```typescript
const characters = await assets.list({ type: "vrm" });
for (const char of characters) {
  console.log(`${char.id} from ${char.mod}`);
}
```

### アニメーションの存在確認

```typescript
const animations = await assets.list({ type: "vrma" });
const hasIdle = animations.some((a) => a.id === "vrma:idle-maid");
```

## 次のステップ

- **[MOD API](./mods-api)** -- インストール済み MOD の一覧と bin コマンドの実行
- **[SDK 概要](./)** -- 全モジュールマップとクイック例

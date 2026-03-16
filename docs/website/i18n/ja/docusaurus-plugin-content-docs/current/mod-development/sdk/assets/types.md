---
sidebar_position: 100
---

# 型定義

### AssetType

```typescript
type AssetType = "vrm" | "vrma" | "sound" | "image" | "html";
```

| 値 | 説明 |
|----|------|
| `vrm` | VRM 3D キャラクターモデル |
| `vrma` | VRM キャラクター用の VRMA アニメーションファイル |
| `sound` | 音声ファイル（効果音、BGM） |
| `image` | 画像ファイル（PNG、JPG など） |
| `html` | WebView コンテンツ用の HTML ファイル |

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

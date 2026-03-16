---
sidebar_position: 2
---

# list

登録されたすべてのアセットを返します。オプションで種類や MOD 名によるフィルタリングが可能です。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| `filter` | `AssetFilter`（オプション） | フィルタ条件：`type` や `mod` |

## Returns

`Promise<AssetInfo[]>`

## Example

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

---
sidebar_position: 2
---

# stamp

画像アセットをスクリーン上の一時的なオーバーレイとして表示します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `asset` | `string` | スタンプ画像のアセット ID（例: `"my-mod:heart"`） |
| `options` | `StampOptions` | オプションの外観設定 |

## 戻り値

`Promise<void>`

## 例

```typescript
// 最小構成 -- デフォルトの位置とサイズで表示
await effects.stamp("my-mod:thumbs-up");

// フルオプション
await effects.stamp("my-mod:heart", {
  x: 100,
  y: 200,
  width: 80,
  height: 80,
  alpha: 0.9,
  duration: 1.5,
});
```

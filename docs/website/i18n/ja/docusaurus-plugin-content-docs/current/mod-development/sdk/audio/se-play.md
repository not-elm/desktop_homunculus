---
sidebar_position: 2
---

# se.play

ワンショットの効果音を再生します。サウンドの再生開始後すぐに呼び出しが返ります。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `asset` | `string` | 効果音のアセット ID（例：`"my-mod:click"`） |
| `options` | `SeOptions` | オプションの再生設定 |

## 戻り値

`Promise<void>`

## 例

```typescript
// アセット ID で効果音を再生
await audio.se.play("my-mod:click");

// 再生オプション付き
await audio.se.play("my-mod:alert", {
  volume: 0.5,
  speed: 1.2,
  panning: -0.5,  // -1.0 = 左、0.0 = 中央、1.0 = 右
});
```

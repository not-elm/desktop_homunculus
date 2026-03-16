---
sidebar_position: 4
---

# bgm.stop

再生中の BGM を停止します。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `options` | [`BgmStopOptions`](./types#bgmstopoptions) | オプションの停止設定（例：フェードアウト）。省略すると即座に停止します。 |

## 戻り値

`Promise<void>`

## 例

```typescript
// 即座に停止
await audio.bgm.stop();

// 2 秒かけてフェードアウト
await audio.bgm.stop({
  fadeOut: { durationSecs: 2.0, easing: "easeOut" },
});
```

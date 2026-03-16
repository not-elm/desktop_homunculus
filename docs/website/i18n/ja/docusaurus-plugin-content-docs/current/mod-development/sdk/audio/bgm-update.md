---
sidebar_position: 7
---

# bgm.update

再生中に音量や速度を変更します。`tween` フィールドを使用してトランジションをアニメーションさせることができます。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `options` | [`BgmUpdateOptions`](./types#bgmupdateoptions) | 更新するパラメーター |

## 戻り値

`Promise<void>`

## 例

```typescript
// 1 秒かけて音量を 0.3 にフェード
await audio.bgm.update({
  volume: 0.3,
  tween: { durationSecs: 1.0, easing: "easeInOut" },
});

// 速度を即座に変更
await audio.bgm.update({ speed: 0.8 });
```

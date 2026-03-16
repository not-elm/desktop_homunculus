---
sidebar_position: 3
---

# bgm.play

BGM を再生します。現在再生中の BGM があれば置き換えます。同時に再生できる BGM トラックは 1 つだけです。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `asset` | `string` | 音楽トラックのアセット ID（例：`"my-mod:battle-theme"`） |
| `options` | [`BgmPlayOptions`](./types#bgmplayoptions) | オプションの再生設定 |

## 戻り値

`Promise<void>`

## 例

```typescript
// デフォルトでループ
await audio.bgm.play("my-mod:battle-theme");

// ワンショット再生、フェードイン付き
await audio.bgm.play("my-mod:intro", {
  loop: false,
  volume: 0.6,
  fadeIn: { durationSecs: 3.0, easing: "easeIn" },
});
```

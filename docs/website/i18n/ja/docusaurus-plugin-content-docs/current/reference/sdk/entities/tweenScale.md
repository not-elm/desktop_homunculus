---
sidebar_position: 9
---

# tweenScale

イージング関数を使用して、エンティティのスケールをターゲットの `[x, y, z]` 値まで指定した時間でスムーズにアニメーションします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entityId` | `number` | トゥイーンするエンティティ ID |
| `request` | [`TweenScaleRequest`](./types#tweenscalerequest) | トゥイーンパラメータ |

## 戻り値

`Promise<void>`

## 使用例

```typescript
await entities.tweenScale(vrmEntity, {
  target: [2, 2, 2],
  durationMs: 800,
  easing: "bounceOut",
  wait: false, // ファイアアンドフォーゲット
});
```

### バウンススケールエフェクト

```typescript
// バウンス付きの素早いスケールアップ
await entities.tweenScale(entity, {
  target: [1.5, 1.5, 1.5],
  durationMs: 300,
  easing: "bounceOut",
  wait: true,
});

// 元のサイズに戻す
await entities.tweenScale(entity, {
  target: [1, 1, 1],
  durationMs: 200,
  easing: "quadraticOut",
  wait: true,
});
```

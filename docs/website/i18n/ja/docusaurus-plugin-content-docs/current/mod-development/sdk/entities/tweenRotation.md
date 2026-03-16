---
sidebar_position: 8
---

# tweenRotation

イージング関数を使用して、エンティティの回転をターゲットのクォータニオン `[x, y, z, w]` まで指定した時間でスムーズにアニメーションします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entityId` | `number` | トゥイーンするエンティティ ID |
| `request` | [`TweenRotationRequest`](./types#tweenrotationrequest) | トゥイーンパラメータ |

## 戻り値

`Promise<void>`

## 使用例

```typescript
await entities.tweenRotation(vrmEntity, {
  target: [0, 0, 0.7071, 0.7071], // Z 軸 90 度
  durationMs: 500,
  easing: "elasticOut",
});
```

### 並列トゥイーン

`wait` を省略（または `wait: false` に設定）すると、複数のトゥイーンを同時に実行できます：

```typescript
entities.tweenPosition(entity, {
  target: [100, 100, 0],
  durationMs: 1000,
  easing: "sineInOut",
});

entities.tweenRotation(entity, {
  target: [0, 0, 0.3827, 0.9239], // Z軸周りに45度
  durationMs: 1000,
  easing: "sineInOut",
});
```

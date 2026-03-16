---
sidebar_position: 7
---

# tweenPosition

イージング関数を使用して、エンティティの位置をターゲットの `[x, y, z]` 値まで指定した時間でスムーズにアニメーションします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entityId` | `number` | トゥイーンするエンティティ ID |
| `request` | [`TweenPositionRequest`](./types#tweenpositionrequest) | トゥイーンパラメータ |

## 戻り値

`Promise<void>`

## 使用例

```typescript
await entities.tweenPosition(vrmEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
  wait: true, // アニメーション完了までブロック
});
```

`wait: true` を設定するとアニメーション完了までブロックします：

```typescript
await entities.tweenPosition(myEntity, {
  target: [0, 200, 0],
  durationMs: 500,
  easing: "cubicOut",
  wait: true,
});
// トゥイーン完了後に処理が続行されます
```

### 画面外からスライドイン

```typescript
const entity = await entities.findByName("MyCharacter");

// 画面左の外から開始し、スライドイン
await entities.setTransform(entity, { translation: [-500, 0, 0] });
await entities.tweenPosition(entity, {
  target: [0, 0, 0],
  durationMs: 800,
  easing: "cubicOut",
  wait: true,
});
```

### 並列トゥイーン

`wait` を省略（または `wait: false` に設定）すると、複数のトゥイーンを同時に実行できます：

```typescript
// 位置と回転が同時にアニメーション
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

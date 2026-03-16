---
sidebar_position: 5
---

# setTransform

エンティティのトランスフォーム（位置、回転、スケール）を更新します。**部分的な**トランスフォームを受け付けます -- 指定したフィールドのみが更新されます。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entity` | `number` | 更新するエンティティ ID |
| `transform` | `Partial<Transform>` | 更新する値を含む部分的なトランスフォームデータ |

## 戻り値

`Promise<void>`

## 使用例

```typescript
// エンティティを上方向に 100 ユニット移動（回転とスケールは変更なし）
await entities.setTransform(vrmEntity, {
  translation: [0, 100, 0],
});

// 3 つのコンポーネントを一度に更新
await entities.setTransform(vrmEntity, {
  translation: [50, 0, -25],
  rotation: [0, 0.707, 0, 0.707], // Y 軸 90 度回転
  scale: [1.5, 1.5, 1.5],
});
```

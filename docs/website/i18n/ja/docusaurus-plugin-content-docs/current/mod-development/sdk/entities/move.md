---
sidebar_position: 6
---

# move

**ワールド**座標または**ビューポート**（スクリーン空間）座標を使用してエンティティを再配置します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entity` | `number` | 移動するエンティティ ID |
| `target` | [`MoveTarget`](./types#movetarget) | ターゲット位置（ワールドまたはビューポート座標） |

## 戻り値

`Promise<void>`

## 使用例

### ワールド座標

3D ワールド空間でエンティティの位置を直接設定します。`z` フィールドはオプションです -- 省略した場合、エンティティは現在の z 値を保持します。

```typescript
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
  z: -2,
});

// 現在の z を保持
await entities.move(vrmEntity, {
  type: "world",
  position: [0, 1.5],
});
```

### ビューポート座標

スクリーンピクセル座標を渡すと、エンジンが自動的にワールド空間に変換します：

```typescript
await entities.move(vrmEntity, {
  type: "viewport",
  position: [500, 300],
});
```

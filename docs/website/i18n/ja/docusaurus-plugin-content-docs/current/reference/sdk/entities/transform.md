---
sidebar_position: 4
---

# transform

エンティティの現在のトランスフォーム（位置、回転、スケール）を取得します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entity` | `number` | トランスフォームを取得するエンティティ ID |

## 戻り値

`Promise<Transform>`

## 使用例

```typescript
const t = await entities.transform(vrmEntity);
console.log("位置:", t.translation); // [x, y, z]
console.log("回転:", t.rotation);    // [x, y, z, w] クォータニオン
console.log("スケール:", t.scale);   // [x, y, z]
```

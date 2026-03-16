---
sidebar_position: 3
---

# name

エンティティ ID に紐づけられた名前を取得します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `entity` | `number` | 名前を取得するエンティティ ID |

## 戻り値

`Promise<string>`

## 使用例

```typescript
const entityName = await entities.name(vrmEntity);
console.log(entityName); // "MyCharacter"
```

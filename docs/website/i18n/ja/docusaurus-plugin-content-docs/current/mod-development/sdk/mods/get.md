---
sidebar_position: 3
---

# get

名前で単一の MOD の詳細情報を取得します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `modName` | `string` | MOD パッケージ名 |

## 戻り値

`Promise<`[`ModInfo`](./types#modinfo)`>`

## 使用例

```typescript
const elmer = await mods.get("elmer");
console.log("Elmer のアセット:", elmer.asset_ids);
```

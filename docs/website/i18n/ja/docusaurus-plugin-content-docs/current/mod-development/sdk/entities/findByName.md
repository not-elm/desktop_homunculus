---
sidebar_position: 2
---

# findByName

人間が読める名前でエンティティを検索します。一致するものが見つからない場合はエラーをスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `name` | `string` | 検索するエンティティの名前 |
| `options` | `FindOptions`（オプション） | 検索オプション（例：サブツリー内に制限） |

## 戻り値

`Promise<number>`

## 使用例

```typescript
const vrmEntity = await entities.findByName("MyCharacter");
```

`root` オプションを渡すと、特定のエンティティの子要素内のみを検索します -- VRM 階層内のボーンを見つける場合に便利です：

```typescript
const headBone = await entities.findByName("head", {
  root: vrmEntity,
});
```

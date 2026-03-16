---
sidebar_position: 6
---

# menus

インストール済み MOD 全体で登録されたすべてのコンテキストメニューエントリを返します。各エントリは MOD の `package.json` の `homunculus.menus` フィールドで宣言されます。

## パラメータ

なし。

## 戻り値

`Promise<ModMenuMetadata[]>`

## 使用例

```typescript
const menuItems = await mods.menus();
for (const item of menuItems) {
  console.log(`${item.modName}: ${item.text} -> ${item.command}`);
}
```

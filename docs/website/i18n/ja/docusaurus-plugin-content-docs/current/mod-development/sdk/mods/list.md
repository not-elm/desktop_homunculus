---
sidebar_position: 2
---

# list

起動時に検出されたすべての MOD のメタデータを返します。

## パラメータ

なし。

## 戻り値

`Promise<`[`ModInfo`](./types#modinfo)`[]>`

## 使用例

```typescript
const allMods = await mods.list();
console.log(`${allMods.length} 個の MOD がインストールされています`);

// bin コマンドを公開している MOD を検索
const withCommands = allMods.filter((m) => m.bin_commands.length > 0);
```

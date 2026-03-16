---
sidebar_position: 3
---

# info

実行中のエンジンインスタンスに関するメタデータを 1 回のリクエストで返します -- バージョン文字列、プラットフォームの詳細、コンパイルされた機能、読み込み済みの全 MOD。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| _(なし)_ | — | — |

## Returns

`Promise<AppInfo>`

## Example

```typescript
const info = await app.info();
console.log(`Engine v${info.version} on ${info.platform.os}/${info.platform.arch}`);
console.log(`Features: ${info.features.join(", ")}`);
console.log(`${info.mods.length} 個の MOD が読み込まれています`);

for (const mod of info.mods) {
  console.log(`  ${mod.name}@${mod.version} — ${mod.binCommands.length} コマンド`);
}
```

---
sidebar_position: 2
---

# list

保存されているすべてのプリファレンスキー名を返します。

## パラメータ

なし。

## 戻り値

`Promise<string[]>`

## 例

```typescript
const keys = await preferences.list();
console.log(`${keys.length} 個のプリファレンスが保存されています`);

for (const key of keys) {
  const value = await preferences.load(key);
  console.log(`${key}:`, value);
}
```

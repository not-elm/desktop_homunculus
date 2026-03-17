---
sidebar_position: 3
---

# load

キーの値を取得します。キーが存在しない場合は `undefined` を返します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `key` | `string` | 保存されたデータの一意識別子 |

## 戻り値

`Promise<V | undefined>`

## 例

```typescript
const theme = await preferences.load<string>("my-mod:theme");
if (theme !== undefined) {
  console.log(`テーマ: ${theme}`);
}

interface Settings {
  volume: number;
  notifications: boolean;
}
const settings = await preferences.load<Settings>("my-mod:settings");
```

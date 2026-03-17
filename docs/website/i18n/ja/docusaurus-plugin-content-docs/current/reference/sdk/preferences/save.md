---
sidebar_position: 4
---

# save

JSON シリアライズ可能な任意の値を指定のキーに保存します。キーが既に存在する場合は、以前の値を上書きします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `key` | `string` | データを保存するための一意識別子 |
| `value` | `V` | 保存するデータ（JSON シリアライズ可能であること） |

## 戻り値

`Promise<void>`

## 例

```typescript
await preferences.save("my-mod:theme", "dark");

await preferences.save("my-mod:settings", {
  volume: 0.8,
  notifications: true,
});
```

:::note キーの命名
他の MOD との衝突を避けるため、`"mod-name:key"` プレフィックスを使用してください。例: `"my-mod:theme"`、`"my-mod:settings"`。
:::

---
sidebar_position: 4
---

# exit

Desktop Homunculus アプリケーションをグレースフルにシャットダウンします。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| _(なし)_ | — | — |

## Returns

`Promise<void>`

## Example

```typescript
await app.exit();
```

:::warning
`app.exit()` はすべての実行中の MOD を含むアプリケーション全体を終了します。注意して使用してください。
:::

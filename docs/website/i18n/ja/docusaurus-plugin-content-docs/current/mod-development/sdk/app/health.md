---
sidebar_position: 2
---

# health

Desktop Homunculus エンジンが到達可能で正常であれば `true` を、そうでなければ `false` を返します。エンジンの起動を待ってから処理を続行する必要があるサービスに便利です。

## Parameters

| Parameter | Type | 説明 |
|-----------|------|-------------|
| _(なし)_ | — | — |

## Returns

`Promise<boolean>`

## Example

```typescript
const alive = await app.health();
if (!alive) {
  console.error("Homunculus エンジンが実行されていません");
}
```

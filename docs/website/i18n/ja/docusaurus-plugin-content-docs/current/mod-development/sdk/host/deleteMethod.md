---
sidebar_position: 10
---

# deleteMethod

自動エラー処理付きの DELETE リクエストを実行します。非 OK レスポンスの場合は [`HomunculusApiError`](./types#homunculusapierror) をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | DELETE リクエストを送信する URL |

## 戻り値

`Promise<Response>`

## 使用例

```typescript
await host.deleteMethod(host.createUrl("vrm/123"));
```

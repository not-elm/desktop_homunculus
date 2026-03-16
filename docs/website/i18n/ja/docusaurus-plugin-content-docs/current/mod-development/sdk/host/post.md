---
sidebar_position: 7
---

# post

JSON ペイロードと自動エラー処理付きの POST リクエストを実行します。非 OK レスポンスの場合は [`HomunculusApiError`](./types#homunculusapierror) をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | POST リクエストを送信する URL |
| `body` | `B`（オプション） | JSON シリアライズされるリクエストボディ |

## 戻り値

`Promise<Response>`

## 使用例

```typescript
await host.post(host.createUrl("vrm"), { asset: "my-mod:character" });
```

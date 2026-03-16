---
sidebar_position: 8
---

# put

JSON ペイロードと自動エラー処理付きの PUT リクエストを実行します。非 OK レスポンスの場合は `HomunculusApiError` をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | PUT リクエストを送信する URL |
| `body` | `B`（オプション） | JSON シリアライズされるリクエストボディ |

## 戻り値

`Promise<Response>`

## 使用例

```typescript
await host.put(host.createUrl("vrm/123/state"), { state: "idle" });
```

---
sidebar_position: 9
---

# patch

JSON ペイロードと自動エラー処理付きの PATCH リクエストを実行します。非 OK レスポンスの場合は `HomunculusApiError` をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | PATCH リクエストを送信する URL |
| `body` | `B`（オプション） | JSON シリアライズされるリクエストボディ |

## 戻り値

`Promise<Response>`

## 使用例

```typescript
await host.patch(host.createUrl("vrm/123/persona"), { profile: "cheerful" });
```

---
sidebar_position: 6
---

# get

指定した URL に自動エラー処理付きの GET リクエストを実行します。非 OK レスポンスの場合は `HomunculusApiError` をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | GET リクエストを送信する URL |

## 戻り値

`Promise<Response>`

## 使用例

```typescript
const response = await host.get(host.createUrl("vrm"));
const vrms = await response.json();
```

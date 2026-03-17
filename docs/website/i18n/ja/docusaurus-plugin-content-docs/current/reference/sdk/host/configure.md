---
sidebar_position: 2
---

# configure

Desktop Homunculus HTTP サーバーの SDK ベース URL を上書きします。デフォルトでは SDK は `http://localhost:3100` に接続します。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `options` | `{ baseUrl: string }` | 設定オプション |

## 戻り値

`void`

## 使用例

```typescript
host.configure({ baseUrl: "http://localhost:4000" });
```

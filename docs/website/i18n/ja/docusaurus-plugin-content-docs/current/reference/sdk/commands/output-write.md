---
sidebar_position: 7
---

# output.write

JSON 結果を stdout に書き込みます。プロセスは **終了しません**。部分的な結果のストリーミングに便利です。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `data` | `unknown` | stdout に JSON として書き込む値 |

## 戻り値

`void`

## 例

```typescript
output.write({ partial: "data" });
// stdout: {"partial":"data"}\n
```

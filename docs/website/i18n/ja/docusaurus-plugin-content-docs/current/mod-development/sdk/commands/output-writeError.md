---
sidebar_position: 8
---

# output.writeError

構造化エラーを stderr に書き込みます。プロセスは **終了しません**。致命的でない警告に便利です。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `code` | `string` | マシン可読なエラーコード |
| `message` | `string` | 人間が読めるエラーの説明 |

## 戻り値

`void`

## 例

```typescript
output.writeError("WARNING", "致命的でない問題");
// stderr: {"code":"WARNING","message":"致命的でない問題"}\n
```

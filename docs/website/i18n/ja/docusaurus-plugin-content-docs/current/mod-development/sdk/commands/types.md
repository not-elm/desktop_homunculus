---
sidebar_position: 100
---

# 型定義

## StdinParseError

`input.parse()` が、stdin が空の場合、無効な JSON を含む場合、または Zod スキーマのバリデーションに失敗した場合にスローされるエラーです。`code` フィールドで失敗段階を識別できます：

| コード | 意味 |
|------|---------|
| `EMPTY_STDIN` | stdin に入力がありません |
| `INVALID_JSON` | stdin の内容が有効な JSON ではありません |
| `VALIDATION_ERROR` | JSON が Zod スキーマに一致しません（`details` フィールドに `ZodError` インスタンスが含まれます） |

```typescript
import { input, output, StdinParseError } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(schema);
  output.succeed(await processData(data));
} catch (err) {
  if (err instanceof StdinParseError) {
    output.fail(err.code, err.message);
  }
  throw err;
}
```

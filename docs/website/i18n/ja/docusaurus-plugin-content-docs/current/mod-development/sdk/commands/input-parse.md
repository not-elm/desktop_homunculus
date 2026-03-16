---
sidebar_position: 2
---

# input.parse

stdin から JSON を読み取り、Zod スキーマでバリデーションします。バリデーション済みの型付きオブジェクトを返します。

内部的に 3 つのステップを実行します：
1. `input.read()` で stdin 全体を読み取り
2. 生の文字列を JSON としてパース
3. パースされたオブジェクトを提供された Zod スキーマでバリデーション

いずれかのステップが失敗すると [`StdinParseError`](./types#stdinparseerror) がスローされます。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `schema` | `ZodType<T>` | パースされた JSON をバリデーションするための Zod スキーマ |

## 戻り値

`Promise<T>`

## 例

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({
    entity: z.number(),
    text: z.union([z.string(), z.array(z.string())]),
    speaker: z.number().default(0),
  }),
);
```

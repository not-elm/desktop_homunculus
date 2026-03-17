---
sidebar_position: 6
---

# output.fail

構造化エラーを stderr に書き込み、プロセスを終了します。デフォルトの終了コードは `1` です。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `code` | `string` | マシン可読なエラーコード（例：`"NOT_FOUND"`、`"TIMEOUT"`） |
| `message` | `string` | 人間が読めるエラーの説明 |
| `exitCode` | `number` | プロセスの終了コード（デフォルト：`1`） |

## 戻り値

`never`

## 例

```typescript
import { output } from "@hmcs/sdk/commands";

output.fail("NOT_FOUND", "エンティティが存在しません");
// stderr: {"code":"NOT_FOUND","message":"エンティティが存在しません"}\n
// プロセスは終了コード 1 で終了
```

---
sidebar_position: 5
---

# output.succeed

JSON 結果を stdout に書き込み、終了コード 0 でプロセスを終了します。成功した bin コマンドの最後の呼び出しとして使用してください。

## パラメーター

| パラメーター | 型 | 説明 |
|-----------|------|-------------|
| `data` | `unknown` | stdout に JSON として書き込むオプションの値 |

## 戻り値

`never`

## 例

```typescript
import { output } from "@hmcs/sdk/commands";

output.succeed({ greeting: `Hello, ${data.name}!` });
// stdout: {"greeting":"Hello, World!"}\n
// プロセスは終了コード 0 で終了
```

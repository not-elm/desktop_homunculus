---
sidebar_position: 11
---

# postStream

POST リクエストを送信し、パースされた NDJSON オブジェクトが到着するたびに yield する非同期ジェネレータを返します。非 OK レスポンスの場合は `HomunculusApiError` をスロー、NDJSON 行がパースできない場合は `HomunculusStreamError` をスローします。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|------|
| `url` | `URL` | POST リクエストを送信する URL |
| `body` | `unknown`（オプション） | JSON シリアライズされるリクエストボディ |
| `signal` | `AbortSignal`（オプション） | キャンセル用のシグナル |

## 戻り値

`AsyncGenerator<T>`

## 使用例

```typescript
import { host, type HomunculusStreamError } from "@hmcs/sdk";

const stream = host.postStream<{ type: string; data: string }>(
  host.createUrl("commands/execute"),
  { command: "build" },
);

for await (const event of stream) {
  console.log(event);
}
```

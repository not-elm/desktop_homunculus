---
sidebar_position: 4
---

# send

`signals.send<V>(signal, payload)` は、そのチャンネルのすべてのアクティブな購読者に JSON ペイロードをブロードキャストします。

購読者がいない場合、メッセージは黙って破棄されます。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `signal` | `string` | ブロードキャストするシグナルチャンネル名 |
| `payload` | `V` | すべての購読者に送信するデータ |

## 戻り値

`Promise<void>`

## 例

```typescript
import { signals } from "@hmcs/sdk";

await signals.send("my-mod:chat", {
  sender: "bot",
  text: "Hello from the server!",
});
```

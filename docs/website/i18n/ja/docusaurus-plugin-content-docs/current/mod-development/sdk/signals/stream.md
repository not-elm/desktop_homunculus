---
sidebar_position: 3
---

# stream

`signals.stream<V>(signal, callback)` は永続的な SSE 接続を開き、指定チャンネルにメッセージが届くたびに `callback` を呼び出します。不要になったら閉じる必要がある `EventSource` インスタンスを返します。

コールバックは `async` にすることもできます。コールバック内のエラーはキャッチされ、コンソールに出力されます。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `signal` | `string` | 購読するシグナルチャンネル名 |
| `callback` | `(payload: V) => void \| Promise<void>` | 受信したペイロードを処理するコールバック関数 |

## 戻り値

`EventSource` -- SSE 接続インスタンス。購読を解除するには `.close()` を呼び出します。

## 例

```typescript
import { signals } from "@hmcs/sdk";

interface ChatMessage {
  sender: string;
  text: string;
}

const es = signals.stream<ChatMessage>("my-mod:chat", (msg) => {
  console.log(`${msg.sender}: ${msg.text}`);
});

// 不要になったら閉じる
es.close();
```

コールバックを `async` にすることもできます：

```typescript
const es = signals.stream<{ url: string }>("my-mod:fetch", async (payload) => {
  const res = await fetch(payload.url);
  console.log(await res.text());
});
```

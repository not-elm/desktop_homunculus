---
sidebar_position: 1
---

# signals

Server-Sent Events（SSE）によるクロスプロセスの Pub/Sub メッセージングです。シグナルを使うことで、MOD サービス、MOD コマンド、WebView 間でリアルタイムに通信できます。

## インポート

```typescript
import { signals } from "@hmcs/sdk";
```

## 関数

| 関数 | 説明 |
|----------|-------------|
| [list](./list) | すべてのアクティブなシグナルチャンネルと購読者数を返します |
| [stream](./stream) | 永続的な SSE 接続を開き、メッセージ受信のたびにコールバックを呼び出します |
| [send](./send) | チャンネルのすべてのアクティブな購読者に JSON ペイロードをブロードキャストします |

参照: [型定義](./types)

---
title: "シグナル"
sidebar_position: 9
---

# シグナル

Server-Sent Events（SSE）によるクロスプロセスの Pub/Sub メッセージングです。シグナルを使うことで、MOD サービス、bin コマンド、WebView 間でリアルタイムに通信できます。

## インポート

```typescript
import { signals } from "@hmcs/sdk";
```

## 購読（Subscribe）

`signals.stream<V>(signal, callback)` は永続的な SSE 接続を開き、指定チャンネルにメッセージが届くたびに `callback` を呼び出します。不要になったら閉じる必要がある `EventSource` インスタンスを返します。

```typescript
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

コールバックは `async` にすることもできます。コールバック内のエラーはキャッチされ、コンソールに出力されます。

```typescript
const es = signals.stream<{ url: string }>("my-mod:fetch", async (payload) => {
  const res = await fetch(payload.url);
  console.log(await res.text());
});
```

## 配信（Publish）

`signals.send<V>(signal, payload)` は、そのチャンネルのすべてのアクティブな購読者に JSON ペイロードをブロードキャストします。

```typescript
await signals.send("my-mod:chat", {
  sender: "bot",
  text: "Hello from the server!",
});
```

購読者がいない場合、メッセージは黙って破棄されます。

## チャンネル一覧

`signals.list()` はすべてのアクティブなシグナルチャンネルと購読者数を返します。デバッグや、送信前に誰かがリッスンしているかの確認に便利です。

```typescript
const channels = await signals.list();
for (const ch of channels) {
  console.log(`${ch.signal}: ${ch.subscribers} subscribers`);
}
```

## 例：リアルタイム同期

シグナルを使って MOD のサービスと WebView UI を同期させる一般的なパターンです。

**サービス**（Node.js で実行）：

```typescript
import { signals, Vrm } from "@hmcs/sdk";

// WebView からのコマンドを待ち受ける
signals.stream<{ action: string }>("my-mod:ui-cmd", async (cmd) => {
  if (cmd.action === "wave") {
    const vrm = await Vrm.findByName("MyAvatar");
    await vrm.playVrma({ asset: "my-mod:wave" });
  }
});
```

**WebView コード**（ブラウザで実行）：

```typescript
import { signals } from "@hmcs/sdk";

// サービスにコマンドを送信
document.getElementById("wave-btn")?.addEventListener("click", () => {
  signals.send("my-mod:ui-cmd", { action: "wave" });
});
```

## 型定義

### `SignalChannelInfo`

`signals.list()` から返されます。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `signal` | `string` | シグナルチャンネル名 |
| `subscribers` | `number` | アクティブな購読者数 |

## 次のステップ

- **[WebViews](./webviews)** -- スクリプトと WebView 間のクロスプロセス Pub/Sub メッセージング
- **[オーディオ](./audio)** -- 効果音と BGM

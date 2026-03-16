---
sidebar_position: 2
---

# list

`signals.list()` はすべてのアクティブなシグナルチャンネルと購読者数を返します。デバッグや、送信前に誰かがリッスンしているかの確認に便利です。

## パラメータ

なし。

## 戻り値

`Promise<`[`SignalChannelInfo`](./types#signalchannelinfo)`[]>`

## 例

```typescript
import { signals } from "@hmcs/sdk";

const channels = await signals.list();
for (const ch of channels) {
  console.log(`${ch.signal}: ${ch.subscribers} subscribers`);
}
```

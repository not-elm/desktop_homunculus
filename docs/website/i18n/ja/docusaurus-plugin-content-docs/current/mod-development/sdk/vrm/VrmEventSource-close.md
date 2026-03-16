---
title: "VrmEventSource.close"
sidebar_position: 34
---

# VrmEventSource.close

```typescript
import { Vrm } from "@hmcs/sdk";
```

`eventSource.close()` は SSE 接続を閉じ、イベントの受信を停止します。

```typescript
const eventSource = character.events();
// ... リスナーを登録 ...

// 完了したら：
eventSource.close();
```

[`VrmEventSource`](./types#vrmeventsource) は `Disposable` も実装しているため、TypeScript の `using` 宣言を使用して自動的にクリーンアップできます：

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // ここで eventSource.close() が自動的に呼び出されます
}
```

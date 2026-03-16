---
title: "Vrm.streamMetadata"
sidebar_position: 8
---

# Vrm.streamMetadata

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.streamMetadata(callback)` は Server-Sent Events ストリームを開き、現在存在するすべての VRM と今後作成されるすべての VRM に対して `VrmMetadata` でコールバックを実行します。完了時に閉じることができる `EventSource` を返します。

```typescript
const es = Vrm.streamMetadata((metadata) => {
  console.log(`VRM appeared: ${metadata.name} (entity: ${metadata.entity})`);
});

// ストリーミングを停止
es.close();
```

[`Vrm.stream`](./stream) とは異なり、このコールバックは `Vrm` ラッパーインスタンスではなく生の `VrmMetadata`（名前とエンティティ ID）を受け取ります。低レベルの制御が必要な場合や `Vrm` インスタンスを自分で構築したい場合に使用します。

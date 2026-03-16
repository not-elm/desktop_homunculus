---
title: "Vrm.stream"
sidebar_position: 9
---

# Vrm.stream

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.stream(callback)` は現在存在するすべての VRM と、今後作成される VRM に対してコールバックを実行します。完了時に閉じることができる `EventSource` を返します。

```typescript
const es = Vrm.stream(async (vrm) => {
  const name = await vrm.name();
  console.log(`VRM appeared: ${name} (entity: ${vrm.entity})`);
});

// ストリーミングを停止
es.close();
```

どの MOD がスポーンしたかに関係なく、シーンに現れるすべてのキャラクターに反応する必要がある MOD に便利です。

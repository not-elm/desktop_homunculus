---
title: "modifyMouth"
sidebar_position: 31
---

# modifyMouth

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.modifyMouth(weights)` はリップシンク用の口の表情を設定します。未指定の口の表情は 0.0 にリセットされますが、口以外のオーバーライド（`happy` や `blink` など）は維持されます。

```typescript
// 「あ」の音の口の形状を設定
await character.modifyMouth({ aa: 0.8 });

// 「お」の音に変更 -- aa は 0 にリセット、他のオーバーライドは維持
await character.modifyMouth({ oh: 1.0 });

// 口を閉じる -- すべての口の表情が 0 にリセット
await character.modifyMouth({});
```

この分離により、リップシンクを感情表現とは独立して制御できます。

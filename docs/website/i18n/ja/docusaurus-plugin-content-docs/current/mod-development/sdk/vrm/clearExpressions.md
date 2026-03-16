---
title: "clearExpressions"
sidebar_position: 30
---

# clearExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.clearExpressions()` はすべての表情オーバーライドを削除し、VRMA アニメーションに完全な制御を戻します。

```typescript
await character.clearExpressions();
```

これは、スクリプトによる表情シーケンスからアニメーション駆動の表情に戻すときに便利です。

---
title: "unlinkVrm"
sidebar_position: 18
---

# unlinkVrm

WebView から VRM リンクを削除し、フリーフローティングにします。

```typescript
async unlinkVrm(): Promise<void>
```

## 例

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// リンク
await webview.setLinkedVrm(vrm);

// リンク解除
await webview.unlinkVrm();
```

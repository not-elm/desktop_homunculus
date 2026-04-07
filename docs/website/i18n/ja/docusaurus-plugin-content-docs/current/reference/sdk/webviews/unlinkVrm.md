---
title: "unlinkPersona"
sidebar_position: 18
---

# unlinkPersona

WebView からペルソナリンクを削除し、フリーフローティングにします。

```typescript
async unlinkPersona(): Promise<void>
```

## 例

```typescript
import { persona } from "@hmcs/sdk";

const p = await persona.load("alice");

// リンク
await webview.setLinkedPersona(p.id);

// リンク解除
await webview.unlinkPersona();
```

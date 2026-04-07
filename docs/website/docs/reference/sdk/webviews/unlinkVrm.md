---
title: "unlinkPersona"
sidebar_position: 18
---

# unlinkPersona

Removes the persona link from this webview, making it free-floating.

```typescript
async unlinkPersona(): Promise<void>
```

## Example

```typescript
import { persona } from "@hmcs/sdk";

const p = await persona.load("alice");

// Link
await webview.setLinkedPersona(p.id);

// Unlink
await webview.unlinkPersona();
```

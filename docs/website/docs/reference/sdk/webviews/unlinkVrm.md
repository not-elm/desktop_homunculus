---
title: "unlinkVrm"
sidebar_position: 18
---

# unlinkVrm

Removes the VRM link from this webview, making it free-floating.

```typescript
async unlinkVrm(): Promise<void>
```

## Example

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// Link
await webview.setLinkedVrm(vrm);

// Unlink
await webview.unlinkVrm();
```

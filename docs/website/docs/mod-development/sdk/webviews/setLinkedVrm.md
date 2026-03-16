---
title: "setLinkedVrm"
sidebar_position: 17
---

# setLinkedVrm

Links this webview to a VRM character so it follows the character's position.

```typescript
async setLinkedVrm(vrm: Vrm): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `vrm` | `Vrm` | The VRM instance to link to this webview |

## Example

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// Link
await webview.setLinkedVrm(vrm);
```

To remove the link, use [`unlinkVrm()`](./unlinkVrm).

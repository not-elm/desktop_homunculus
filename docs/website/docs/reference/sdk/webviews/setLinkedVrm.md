---
title: "setLinkedPersona"
sidebar_position: 17
---

# setLinkedPersona

Links this webview to a persona so it follows the persona's character position.

```typescript
async setLinkedPersona(personaId: string): Promise<void>
```

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `personaId` | `string` | The persona ID to link to this webview |

## Example

```typescript
import { persona } from "@hmcs/sdk";

const p = await persona.load("alice");

// Link
await webview.setLinkedPersona(p.id);
```

To remove the link, use [`unlinkPersona()`](./unlinkVrm).

---
title: "linkedPersona"
sidebar_position: 16
---

# linkedPersona

Gets the persona linked to this webview.

```typescript
async linkedPersona(): Promise<Persona | undefined>
```

## Returns

A `Promise` that resolves to the linked `Persona` instance, or `undefined` if no persona is linked.

## Example

```typescript
import { persona } from "@hmcs/sdk";

// Query the linked persona
const linked = await webview.linkedPersona();
// linked is a Persona instance, or undefined if not linked
```

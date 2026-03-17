---
title: "linkedVrm"
sidebar_position: 16
---

# linkedVrm

Gets the VRM linked to this webview.

```typescript
async linkedVrm(): Promise<Vrm | undefined>
```

## Returns

A `Promise` that resolves to the linked `Vrm` instance, or `undefined` if no VRM is linked.

## Example

```typescript
import { Vrm } from "@hmcs/sdk";

// Query the linked VRM
const linked = await webview.linkedVrm();
// linked is a Vrm instance, or undefined if not linked
```

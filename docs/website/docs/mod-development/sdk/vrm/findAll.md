---
title: "Vrm.findAll"
sidebar_position: 5
---

# Vrm.findAll

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAll()` returns `Vrm` instances for all currently loaded characters.

```typescript
// Get Vrm instances for all loaded characters
const characters = await Vrm.findAll();
for (const vrm of characters) {
  const name = await vrm.name();
  console.log(`${name} (entity: ${vrm.entity})`);
}
```

---
title: "Vrm.stream"
sidebar_position: 9
---

# Vrm.stream

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.stream(callback)` fires for every VRM that currently exists and for any VRM that is created in the future. It returns an `EventSource` that you can close when done.

```typescript
const es = Vrm.stream(async (vrm) => {
  const name = await vrm.name();
  console.log(`VRM appeared: ${name} (entity: ${vrm.entity})`);
});

// Later, stop streaming
es.close();
```

This is useful for MODs that need to react to any character appearing in the scene, regardless of which MOD spawned it.

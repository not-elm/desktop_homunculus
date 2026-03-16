---
title: "Vrm.streamMetadata"
sidebar_position: 8
---

# Vrm.streamMetadata

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.streamMetadata(callback)` opens a Server-Sent Events stream and fires the callback with `VrmMetadata` for every VRM that currently exists and for any VRM created in the future. Returns an `EventSource` that you can close when done.

```typescript
const es = Vrm.streamMetadata((metadata) => {
  console.log(`VRM appeared: ${metadata.name} (entity: ${metadata.entity})`);
});

// Later, stop streaming
es.close();
```

Unlike [`Vrm.stream`](./stream), this callback receives raw `VrmMetadata` (name and entity ID) instead of `Vrm` wrapper instances. Use it when you need lower-level control or want to construct `Vrm` instances yourself.

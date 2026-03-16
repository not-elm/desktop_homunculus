---
title: "Vrm.findAllDetailed"
sidebar_position: 7
---

# Vrm.findAllDetailed

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAllDetailed()` returns full runtime state for every loaded VRM -- including transform, expressions, animations, persona, and linked webviews.

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: state=${s.state}`);
  console.log(`  Position: (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
  console.log(`  Animations: ${s.animations.length} active`);
  console.log(`  Expressions: ${s.expressions.expressions.length} defined`);
}
```

Returns an array of [`VrmSnapshot`](./types) objects. Use [`Vrm.findAll`](./findAll) if you only need `Vrm` instances without the detailed state.

---
title: "Vrm.findAllEntities"
sidebar_position: 6
---

# Vrm.findAllEntities

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAllEntities()` returns the raw entity IDs of all currently loaded VRM instances without wrapping them in `Vrm` objects.

```typescript
const entityIds = await Vrm.findAllEntities();
console.log(`Found ${entityIds.length} VRM entities`);
```

Use this when you only need the entity IDs and want to avoid the overhead of constructing `Vrm` wrapper objects. Use [`Vrm.findAll`](./findAll) when you need to call instance methods on the results.

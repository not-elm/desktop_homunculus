---
title: "displays"
sidebar_position: 12
---

# displays

Query connected monitors -- their identifiers, names, and screen-space frame rectangles.

## Import

```typescript
import { displays } from "@hmcs/sdk";
```

## List Displays

`displays.findAll()` returns an array of `GlobalDisplay` objects, one per connected monitor.

```typescript
const allDisplays = await displays.findAll();
console.log(`Found ${allDisplays.length} display(s)`);

for (const d of allDisplays) {
  console.log(`${d.title} (id: ${d.id})`);
  console.log(`  Frame: [${d.frame.min}] - [${d.frame.max}]`);
}
```

**Signature:**

```typescript
displays.findAll(): Promise<GlobalDisplay[]>
```

## Types

### GlobalDisplay

```typescript
interface GlobalDisplay {
  /** Unique display identifier. */
  id: number;
  /** Human-readable display name. */
  title: string;
  /** Display frame rectangle in screen coordinates. */
  frame: Rect;
}
```

See [Coordinates](./coordinates) for `GlobalDisplay` and related type definitions.

## Next Steps

- **[Coordinates](./coordinates)** -- Convert between screen-space and world-space positions.

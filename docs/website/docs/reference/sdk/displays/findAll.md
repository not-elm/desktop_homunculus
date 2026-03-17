---
sidebar_position: 2
---

# findAll

Returns an array of [`GlobalDisplay`](../coordinates/types#globaldisplay) objects, one per connected monitor.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| _(none)_ | — | — |

## Returns

`Promise<`[`GlobalDisplay`](../coordinates/types#globaldisplay)`[]>`

## Example

```typescript
const allDisplays = await displays.findAll();
console.log(`Found ${allDisplays.length} display(s)`);

for (const d of allDisplays) {
  console.log(`${d.title} (id: ${d.id})`);
  console.log(`  Frame: [${d.frame.min}] - [${d.frame.max}]`);
}
```

The `GlobalDisplay` type is defined in [Coordinates Types](../coordinates/types#globaldisplay).

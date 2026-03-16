---
sidebar_position: 2
---

# health

Returns `true` if the Desktop Homunculus engine is reachable and healthy, `false` otherwise. Useful for services that need to wait for the engine before proceeding.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| _(none)_ | — | — |

## Returns

`Promise<boolean>`

## Example

```typescript
const alive = await app.health();
if (!alive) {
  console.error("Homunculus engine is not running");
}
```

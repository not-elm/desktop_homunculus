---
sidebar_position: 3
---

# name

Retrieve the name attached to an entity ID.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entity` | `number` | The entity ID to get the name for |

## Returns

`Promise<string>`

## Example

```typescript
const entityName = await entities.name(vrmEntity);
console.log(entityName); // "MyCharacter"
```

---
sidebar_position: 2
---

# findByName

Look up an entity by its human-readable name. Throws if no match is found.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | `string` | The name of the entity to find |
| `options` | [`FindOptions`](./types#findoptions) (optional) | Search options (e.g., restrict to a subtree) |

## Returns

`Promise<number>`

## Example

```typescript
const vrmEntity = await entities.findByName("MyCharacter");
```

Pass a `root` option to search only within the children of a specific entity -- useful for finding bones inside a VRM hierarchy:

```typescript
const headBone = await entities.findByName("head", {
  root: vrmEntity,
});
```

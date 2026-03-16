---
sidebar_position: 1
---

# entities

Query and manipulate Bevy ECS entities by name. Entities are the building blocks of the 3D scene -- VRM characters, cameras, webviews, and spawned objects are all entities with a numeric ID, an optional name, and a transform (position, rotation, scale).

## Import

```typescript
import { entities } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [findByName](./findByName) | Look up an entity by its human-readable name |
| [name](./name) | Retrieve the name attached to an entity ID |
| [transform](./transform) | Read an entity's position, rotation, and scale |
| [setTransform](./setTransform) | Write a (partial) transform to an entity |
| [move](./move) | Reposition an entity using world or viewport coordinates |
| [tweenPosition](./tweenPosition) | Smoothly animate an entity's position |
| [tweenRotation](./tweenRotation) | Smoothly animate an entity's rotation |
| [tweenScale](./tweenScale) | Smoothly animate an entity's scale |

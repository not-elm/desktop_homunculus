---
sidebar_position: 8
---

# tweenRotation

Smoothly animate an entity's rotation to a target quaternion `[x, y, z, w]` over a given duration using an easing function.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entityId` | `number` | The entity ID to tween |
| `request` | `TweenRotationRequest` | Tween parameters |

## Returns

`Promise<void>`

## Example

```typescript
await entities.tweenRotation(vrmEntity, {
  target: [0, 0, 0.7071, 0.7071], // 90 degrees around Z axis
  durationMs: 500,
  easing: "elasticOut",
});
```

### Parallel tweens

Omit `wait` (or set `wait: false`) to run multiple tweens simultaneously:

```typescript
entities.tweenPosition(entity, {
  target: [100, 100, 0],
  durationMs: 1000,
  easing: "sineInOut",
});

entities.tweenRotation(entity, {
  target: [0, 0, 0.3827, 0.9239], // 45 degrees around Z
  durationMs: 1000,
  easing: "sineInOut",
});
```

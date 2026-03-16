---
sidebar_position: 7
---

# tweenPosition

Smoothly animate an entity's position to a target `[x, y, z]` value over a given duration using an easing function.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entityId` | `number` | The entity ID to tween |
| `request` | `TweenPositionRequest` | Tween parameters |

## Returns

`Promise<void>`

## Example

```typescript
await entities.tweenPosition(vrmEntity, {
  target: [100, 50, 0],
  durationMs: 1000,
  easing: "quadraticInOut",
  wait: true, // block until animation finishes
});
```

Set `wait: true` to block until the animation completes:

```typescript
await entities.tweenPosition(myEntity, {
  target: [0, 200, 0],
  durationMs: 500,
  easing: "cubicOut",
  wait: true,
});
// Execution continues only after the tween finishes
```

### Slide in from offscreen

```typescript
const entity = await entities.findByName("MyCharacter");

// Start offscreen to the left, then slide in
await entities.setTransform(entity, { translation: [-500, 0, 0] });
await entities.tweenPosition(entity, {
  target: [0, 0, 0],
  durationMs: 800,
  easing: "cubicOut",
  wait: true,
});
```

### Parallel tweens

Omit `wait` (or set `wait: false`) to run multiple tweens simultaneously:

```typescript
// Position and rotation animate at the same time
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

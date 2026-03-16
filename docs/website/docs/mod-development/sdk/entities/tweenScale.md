---
sidebar_position: 9
---

# tweenScale

Smoothly animate an entity's scale to a target `[x, y, z]` value over a given duration using an easing function.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `entityId` | `number` | The entity ID to tween |
| `request` | `TweenScaleRequest` | Tween parameters |

## Returns

`Promise<void>`

## Example

```typescript
await entities.tweenScale(vrmEntity, {
  target: [2, 2, 2],
  durationMs: 800,
  easing: "bounceOut",
  wait: false, // fire-and-forget
});
```

### Bounce scale effect

```typescript
// Quick scale-up with bounce
await entities.tweenScale(entity, {
  target: [1.5, 1.5, 1.5],
  durationMs: 300,
  easing: "bounceOut",
  wait: true,
});

// Return to normal
await entities.tweenScale(entity, {
  target: [1, 1, 1],
  durationMs: 200,
  easing: "quadraticOut",
  wait: true,
});
```

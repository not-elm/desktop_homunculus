# World to Global Viewport

Converts 3D world coordinates to global viewport (screen) coordinates.

This transformation projects 3D positions in the world onto screen space, allowing you to position UI elements, effects,
or webviews relative to 3D objects like VRM characters or scene elements.

## Parameters

- `world` (optional): 3D world coordinates to convert
    - `x`: X coordinate in world space
    - `y`: Y coordinate in world space
    - `z`: Z coordinate in world space
    - If not provided, uses the world origin (0, 0, 0)

## Returns

Promise that resolves to a `GlobalViewport` object with `x` and `y` screen coordinates.

## Examples

### Position UI Above VRM Character

```typescript
// Position UI above a VRM character
const vrm = await Vrm.findByName('MyCharacter');
const vrmTransform = await entities.transform(vrm.entity);

const screenPos = await cameras.worldToGlobalViewport({
    x: vrmTransform.translation[0],
    y: vrmTransform.translation[1] + 1.8, // Above character's head
    z: vrmTransform.translation[2]
});

await Webview.open('character-info', {
    position: [screenPos.x - 100, screenPos.y - 50], // Center the 200px wide webview
    resolution: [200, 100],
    transparent: true
});
```

## Related Functions

- [`globalViewportToWorld2d()`](./globalViewportToWorld2d.md) - Convert screen coordinates to 2D world coordinates
- [`entities.transform()`](../entities/transform.md) - Get entity position in world space
- [`webviews.open()`](../webviews/open.md) - Create positioned webviews
- [`effects.stamp()`](../effects/stamp.md) - Display effects at screen positions

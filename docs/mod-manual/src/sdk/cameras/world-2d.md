# Global Viewport to World 2D Conversion

Converts global viewport coordinates to 2D world space coordinates.

This transformation maps screen-space coordinates (like mouse positions or UI element positions) into the 2D coordinate
system of the 3D world.
This is useful for placing objects or effects at screen positions within the 3D scene.

## Parameters

- `viewport` (optional): Screen coordinates to convert
    - `x`: X coordinate in screen space
    - `y`: Y coordinate in screen space
    - If not provided, uses the center of the screen

## Returns

Promise that resolves to a `World2d` object with `x` and `y` coordinates in world space.

## Examples

### Convert Mouse Click to World Position

```typescript
// Convert mouse click to world position
document.addEventListener('click', async (event) => {
    const worldPos = await cameras.globalViewportToWorld2d({
        x: event.clientX,
        y: event.clientY
    });

    console.log(`Clicked at world position:`, worldPos);

    // Spawn an effect at the clicked position
    await effects.stamp('click-indicator/marker.png', {
        position: [worldPos.x, worldPos.y]
    });
});
```

## Related Functions

- [`worldToGlobalViewport()`](./worldToGlobalViewport.md) - Convert 3D world coordinates to screen coordinates
- [`entities.setTransform()`](../entities/setTransform.md) - Set entity position using world coordinates
- [`effects.stamp()`](../effects/stamp.md) - Display effects at world positions

# Cameras API

The Cameras API provides utilities for coordinate system transformations and viewport management. This is essential for
positioning UI elements, placing effects, and converting between screen coordinates and 3D world positions.

## Global Viewport

The global viewport represents the entire screen space across all connected monitors.

## Coordinate Systems

Desktop Homunculus uses multiple coordinate systems:

- **Global Viewport**: Screen-space coordinates relative to the entire desktop
- **World 2D**: 2D coordinates within the 3D world space
- **World 3D**: Full 3D coordinates in world space

## Example

```typescript
// Convert mouse position to world coordinates
const mousePos = {x: 150, y: 200};
const worldPos = await cameras.globalViewportToWorld2d(mousePos);

// Convert VRM position to screen coordinates  
const vrm = await Vrm.findByName('MyCharacter');
const vrmTransform = await entities.transform(vrm.entity);
const screenPos = await cameras.worldToGlobalViewport({
    x: vrmTransform.translation[0],
    y: vrmTransform.translation[1],
    z: vrmTransform.translation[2]
});
```

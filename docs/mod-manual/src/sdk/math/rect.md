# Rect

Represents a 2D rectangle defined by minimum and maximum points. Used throughout the SDK for viewport bounds, UI
regions, effect positioning, and screen area definitions.

## Type Definition

```typescript
interface Rect {
    min: [number, number];
    max: [number, number];
}
```

## Properties

- **min**: Bottom-left corner coordinates as [x, y]
- **max**: Top-right corner coordinates as [x, y]

## Examples

### Basic Usage

```typescript
// Screen viewport definition
const viewport: Rect = {
    min: [0, 0],        // Bottom-left corner
    max: [1920, 1080]   // Top-right corner
};

// UI element bounds
const buttonBounds: Rect = {
    min: [100, 50],     // Left: 100px, Bottom: 50px
    max: [300, 150]     // Right: 300px, Top: 150px
};

// Effect area
const explosionArea: Rect = {
    min: [400, 300],
    max: [600, 500]
};
```

## Related Types

- **[Vec2](./Vec2.md)** - Used for rectangle corners and dimensions
- **[Vec3](./Vec3.md)** - Can be projected to screen space for Rect creation
- **[Transform](./Transform.md)** - 3D entities can be bounded by screen Rects

## Related Documentation

- [Effects](../effects/index.md) - Uses Rect for positioning visual effects
- [Webviews](../webviews/index.md) - Uses Rect concepts for UI bounds
- [Cameras](../cameras/index.md) - Projects 3D coordinates to screen Rects
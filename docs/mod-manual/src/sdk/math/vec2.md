# Vec2

Represents a 2D vector with x and y components. Used throughout the SDK for screen coordinates, UI positioning, 2D
mathematical operations, and coordinate system conversions.

## Type Definition

```typescript
interface Vec2 {
    x: number;
    y: number;
}
```

## Properties

- **x**: The horizontal coordinate or x-component of the vector
- **y**: The vertical coordinate or y-component of the vector

## Examples

### Basic Usage

```typescript
// Screen coordinates
const mousePosition: Vec2 = {x: 150, y: 200};
const screenCenter: Vec2 = {x: 1920 / 2, y: 1080 / 2};

// 2D offset or direction
const offset: Vec2 = {
    x: mousePosition.x - screenCenter.x,
    y: mousePosition.y - screenCenter.y
};
```

## Related Types

- **[Vec3](./Vec3.md)** - 3D vector extension of Vec2
- **[Transform](./Transform.md)** - Uses Vec3 internally for translation
- **[Rect](./Rect.md)** - Uses Vec2 concepts for 2D bounds

## Related Documentation

- [Cameras](../cameras/index.md) - Coordinate system conversions using Vec2
- [Effects](../effects/index.md) - 2D positioning for visual effects
- [Webviews](../webviews/index.md) - UI positioning and layout
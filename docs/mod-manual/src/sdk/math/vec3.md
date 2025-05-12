# Vec3

Represents a 3D vector with x, y, and z components. Used throughout the SDK for 3D positions, directions, mathematical
calculations, and spatial operations in the 3D world space.

## Type Definition

```typescript
interface Vec3 {
    x: number;
    y: number;
    z: number;
}
```

## Properties

- **x**: The x-coordinate or x-component of the vector (left/right)
- **y**: The y-coordinate or y-component of the vector (down/up)
- **z**: The z-coordinate or z-component of the vector (forward/backward)

## Examples

### Basic Usage

```typescript
// 3D world positions
const worldPosition: Vec3 = {x: 10, y: 50, z: -20};
const characterPos: Vec3 = {x: 0, y: 0, z: 100};

// Direction vectors
const upDirection: Vec3 = {x: 0, y: 1, z: 0};
const forwardDirection: Vec3 = {x: 0, y: 0, z: 1};
const rightDirection: Vec3 = {x: 1, y: 0, z: 0};

// Velocity and movement
const velocity: Vec3 = {x: 1.5, y: 0, z: 2.0};
```

## Related Types

- **[Vec2](./Vec2.md)** - 2D vector counterpart
- **[Transform](./Transform.md)** - Uses Vec3 for translation and scale
- **[Rect](./Rect.md)** - 2D bounds complementing 3D vectors

## Related Documentation

- [Entities](../entities/index.md) - 3D object manipulation using Vec3
- [Cameras](../cameras/index.md) - Coordinate conversions with Vec3
- [VRM](../vrm/index.md) - Character positioning and bone operations
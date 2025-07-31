# Math Types and Utilities

The Math module provides TypeScript interfaces and types for mathematical concepts used throughout the Desktop
Homunculus SDK. These types ensure type safety and consistency when working with 3D graphics, spatial calculations, and
geometric operations.

## Overview

All mathematical types in this module are designed to be compatible with Bevy's math system and follow standard 3D
graphics conventions. The types provide a clean, typed interface for:

- **3D Transformations**: Position, rotation, and scale in 3D space
- **Vector Operations**: 2D and 3D vector mathematics
- **Geometric Shapes**: Rectangles and bounds for UI and effects
- **Coordinate Systems**: Consistent representation across different spaces

## Available Types

### Core Types

- **[Transform](./Transform.md)** - Complete 3D transformation with position, rotation, and scale
- **[Vec2](./Vec2.md)** - 2D vector for screen coordinates and 2D math
- **[Vec3](./Vec3.md)** - 3D vector for world positions and directions
- **[Rect](./Rect.md)** - 2D rectangle defined by min/max points

## Coordinate System Conventions

### 3D World Space

- **X-axis**: Left (-) to Right (+)
- **Y-axis**: Down (-) to Up (+)
- **Z-axis**: Forward (+) to Backward (-)
- **Rotation**: Quaternions in [x, y, z, w] format

### 2D Screen Space

- **X-axis**: Left (0) to Right (screen width)
- **Y-axis**: Top (0) to Bottom (screen height)
- **Origin**: Top-left corner of screen

## Related Documentation

- [Cameras](../cameras/index.md) - Coordinate system transformations
- [Entities](../entities/index.md) - 3D object manipulation using transforms
- [Effects](../effects/index.md) - Uses Rect for positioning effects
- [Webviews](../webviews/index.md) - Uses Vec2 for UI positioning
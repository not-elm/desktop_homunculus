/**
 * Mathematical types and interfaces for 3D graphics and spatial calculations.
 *
 * This module provides type definitions for common mathematical concepts used
 * throughout the Desktop Homunculus SDK, including transforms, vectors, and
 * domain-specific request/response types.
 * These types are designed to be compatible with Bevy's math system.
 *
 * @example
 * ```typescript
 * // Working with transforms
 * const transform: TransformArgs = {
 *   translation: [0, 100, 0],
 *   rotation: [0, 0, 0, 1],
 *   scale: [1, 1, 1]
 * };
 *
 * // Working with vectors
 * const position: Vec3 = [10, 20, 30];
 * const screenPos: Vec2 = [1920, 1080];
 * ```
 */

/**
 * Represents a 3D transformation containing position, rotation, and scale.
 *
 * This is the core type for positioning objects in 3D space. All spatial
 * operations in Desktop Homunculus use this transform representation,
 * which is compatible with Bevy's Transform component.
 *
 * @example
 * ```typescript
 * const identity: Transform = {
 *   translation: [0, 0, 0],
 *   rotation: [0, 0, 0, 1],
 *   scale: [1, 1, 1]
 * };
 * ```
 */
export interface Transform {
  /**
   * The position of the entity in world space.
   * Format: [x, y, z] where Y is typically up in Bevy's coordinate system.
   */
  translation: [number, number, number];
  /**
   * The rotation of the entity in world space, represented as a quaternion.
   * Format: [x, y, z, w] where [0, 0, 0, 1] represents no rotation (identity).
   */
  rotation: [number, number, number, number];
  /**
   * The scale of the entity in world space.
   * Format: [x, y, z] where [1, 1, 1] represents normal size.
   */
  scale: [number, number, number];
}

/**
 * Represents a 2D vector as [x, y].
 * Used for screen coordinates, UI positions, and 2D math operations.
 * Compatible with Bevy's Vec2 serialization format.
 */
export type Vec2 = [number, number];

/**
 * Represents a 3D vector as [x, y, z].
 * Used for 3D positions, directions, and mathematical calculations.
 * Compatible with Bevy's Vec3 serialization format.
 */
export type Vec3 = [number, number, number];

/**
 * Represents a quaternion rotation as [x, y, z, w].
 * Compatible with Bevy's Quat serialization format.
 */
export type Quat = [number, number, number, number];

/** Transform arguments for API requests. Partial version of Transform. */
export interface TransformArgs {
  translation?: Vec3;
  rotation?: Quat;
  scale?: Vec3;
}

/** A 2D rectangle defined by minimum and maximum points. */
export interface Rect {
  min: Vec2;
  max: Vec2;
}

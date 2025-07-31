/**
 * Mathematical types and interfaces for 3D graphics and spatial calculations.
 * 
 * This module provides type definitions for common mathematical concepts used
 * throughout the Desktop Homunculus SDK, including transforms, vectors, and rectangles.
 * These types are designed to be compatible with Bevy's math system.
 * 
 * @example
 * ```typescript
 * // Working with transforms
 * const transform: Transform = {
 *   translation: [0, 100, 0],        // Position: 100 units up
 *   rotation: [0, 0, 0, 1],          // Identity rotation (no rotation)
 *   scale: [1, 1, 1]                 // Normal scale
 * };
 * 
 * // Working with vectors
 * const position: Vec3 = { x: 10, y: 20, z: 30 };
 * const screenPos: Vec2 = { x: 1920, y: 1080 };
 * 
 * // Working with rectangles
 * const viewport: Rect = {
 *   min: [0, 0],      // Bottom-left corner
 *   max: [1920, 1080] // Top-right corner
 * };
 * ```
 */

/**
 * Represents a 2D rectangle defined by minimum and maximum points.
 * Used for viewports, UI bounds, and screen regions.
 */
export interface Rect {
    /** Bottom-left corner coordinates [x, y] */
    min: [number, number];
    /** Top-right corner coordinates [x, y] */
    max: [number, number];
}

/**
 * Represents a 3D transformation containing position, rotation, and scale.
 * 
 * This is the core type for positioning objects in 3D space. All spatial
 * operations in Desktop Homunculus use this transform representation,
 * which is compatible with Bevy's Transform component.
 * 
 * @example
 * ```typescript
 * // Identity transform (default position)
 * const identity: Transform = {
 *   translation: [0, 0, 0],
 *   rotation: [0, 0, 0, 1],
 *   scale: [1, 1, 1]
 * };
 * 
 * // Move VRM 50 units forward and 100 units up
 * const newPosition: Partial<Transform> = {
 *   translation: [0, 100, 50]
 * };
 * await entities.setTransform(vrmEntity, newPosition);
 * 
 * // Rotate 90 degrees around Y axis and double the size
 * const rotateAndScale: Partial<Transform> = {
 *   rotation: [0, 0.707, 0, 0.707],  // 90° Y rotation as quaternion
 *   scale: [2, 2, 2]                  // Double the size
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
     * 
     * Common rotations:
     * - Identity: [0, 0, 0, 1]
     * - 90° Y rotation: [0, 0.707, 0, 0.707]
     * - 180° Y rotation: [0, 1, 0, 0]
     */
    rotation: [number, number, number, number];
    /**
     * The scale of the entity in world space.
     * Format: [x, y, z] where [1, 1, 1] represents normal size.
     * Values > 1 make the entity larger, values < 1 make it smaller.
     */
    scale: [number, number, number];
}

/**
 * Represents a 2D vector with x and y components.
 * Used for screen coordinates, UI positions, and 2D math operations.
 * 
 * @example
 * ```typescript
 * const mousePos: Vec2 = { x: 150, y: 200 };
 * const screenCenter: Vec2 = { x: 1920/2, y: 1080/2 };
 * const offset: Vec2 = { 
 *   x: mousePos.x - screenCenter.x,
 *   y: mousePos.y - screenCenter.y 
 * };
 * ```
 */
export interface Vec2 {
    /** The x-coordinate of the vector */
    x: number;
    /** The y-coordinate of the vector */
    y: number;
}

/**
 * Represents a 3D vector with x, y, and z components.
 * Used for 3D positions, directions, and mathematical calculations.
 * 
 * @example
 * ```typescript
 * const worldPosition: Vec3 = { x: 10, y: 50, z: -20 };
 * const direction: Vec3 = { x: 0, y: 1, z: 0 };  // Pointing up
 * const velocity: Vec3 = { x: 1.5, y: 0, z: 2.0 };
 * 
 * // Calculate distance between two points
 * const distance = Math.sqrt(
 *   Math.pow(pos2.x - pos1.x, 2) +
 *   Math.pow(pos2.y - pos1.y, 2) +
 *   Math.pow(pos2.z - pos1.z, 2)
 * );
 * ```
 */
export interface Vec3 {
    /** The x-coordinate of the vector */
    x: number;
    /** The y-coordinate of the vector */
    y: number;
    /** The z-coordinate of the vector */
    z: number;
}
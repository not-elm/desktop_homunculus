import { host } from './host';
import type { Rect, Vec2, Vec3 } from './math';

/**
 * Coordinates API namespace provides coordinate system transformation utilities.
 *
 * Provides utilities for converting between different coordinate spaces used in the
 * Desktop Homunculus 3D environment. This is essential for positioning UI elements,
 * placing effects, and converting between screen coordinates and 3D world positions.
 *
 * Coordinate systems:
 * - **Global Viewport**: Screen-space coordinates relative to the entire desktop
 * - **World 2D**: 2D coordinates within the 3D world space
 * - **World 3D**: Full 3D coordinates in world space
 *
 * @example
 * ```typescript
 * // Convert mouse position to 3D world coordinates
 * const worldPos2D = await coordinates.toWorld({ x: 150, y: 200 });
 *
 * // Convert 3D object position to screen coordinates
 * const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
 * ```
 */
export namespace coordinates {
  /** 2D world coordinates within the 3D scene. */
  export type World2d = Vec2;

  /** Full 3D world coordinates with x, y, and z components. */
  export type World3d = Vec3;

  /**
   * Converts global viewport coordinates to 2D world space coordinates.
   *
   * This transformation maps screen-space coordinates (like mouse positions or
   * UI element positions) into the 2D coordinate system of the 3D world.
   *
   * @param viewport - Screen coordinates to convert (uses center if not provided)
   * @returns A promise that resolves to the corresponding 2D world coordinates
   *
   * @example
   * ```typescript
   * const worldPos = await coordinates.toWorld({ x: 150, y: 200 });
   * ```
   */
  export async function toWorld(viewport?: { x?: number; y?: number }): Promise<Vec2> {
    const url = host.createUrl('coordinates/to-world', viewport);
    const response = await host.get(url);
    return await response.json();
  }

  /**
   * Converts 3D world coordinates to global viewport (screen) coordinates.
   *
   * This transformation projects 3D positions in the world onto screen space,
   * allowing you to position UI elements, effects, or webviews relative to
   * 3D objects like VRM characters or scene elements.
   *
   * @param world - 3D world coordinates to convert (uses origin if not provided)
   * @returns A promise that resolves to the corresponding screen coordinates
   *
   * @example
   * ```typescript
   * const screenPos = await coordinates.toViewport({ x: 0, y: 1.5, z: 0 });
   * ```
   */
  export async function toViewport(world?: {
    x?: number;
    y?: number;
    z?: number;
  }): Promise<GlobalViewport> {
    const url = host.createUrl('coordinates/to-viewport', world);
    const response = await host.get(url);
    return await response.json();
  }
}

/** Information about a connected display/monitor. */
export interface GlobalDisplay {
  /** Unique display identifier. */
  id: number;
  /** Human-readable display name. */
  title: string;
  /** Display frame rectangle in screen coordinates. */
  frame: Rect;
}

/** Global viewport coordinates (screen-space position) as [x, y]. */
export type GlobalViewport = [number, number];

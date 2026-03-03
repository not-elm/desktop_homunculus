import { host } from "./host";
import { type Transform, type Vec2 } from "./math";

/**
 * Entities API namespace for managing ECS (Entity Component System) entities.
 *
 * In Desktop Homunculus, everything is represented as entities in Bevy's ECS system.
 * This includes VRM models, bones, UI elements, and other game objects. This namespace
 * provides core functionality for finding entities by name and manipulating their
 * transforms (position, rotation, scale).
 *
 * Key concepts:
 * - **Entity**: A unique identifier in the ECS system
 * - **Name**: Human-readable identifier for entities
 * - **Transform**: Position, rotation, and scale data
 * - **Hierarchy**: Entities can have parent-child relationships
 *
 * @example
 * ```typescript
 * // Find a VRM entity by name
 * const vrmEntity = await entities.findByName("MyCharacter");
 *
 * // Get the current transform (position, rotation, scale)
 * const transform = await entities.transform(vrmEntity);
 * console.log("Position:", transform.translation);
 *
 * // Move the VRM to a new position
 * await entities.setTransform(vrmEntity, {
 *   translation: [100, 0, 50]
 * });
 *
 * // Find a bone within a specific VRM
 * const headBone = await entities.findByName("head", { root: vrmEntity });
 * ```
 */
export namespace entities {

    /**
     * Options for entity search operations.
     */
    export interface FindOptions {
        /**
         * Optional root entity to search within.
         * If specified, the search will be limited to children of this entity.
         * If not specified, the search will be global across all entities.
         */
        root?: number;
    }

    /**
     * Gets the current transform (position, rotation, scale) of an entity.
     *
     * The transform defines where the entity is positioned in 3D space,
     * how it's rotated, and its scale factor.
     *
     * @param entity - The entity ID to get the transform for
     * @returns A promise that resolves to the entity's transform data
     *
     * @example
     * ```typescript
     * const vrmEntity = await entities.findByName("MyCharacter");
     * const transform = await entities.transform(vrmEntity);
     *
     * console.log("Position:", transform.translation);
     * console.log("Rotation:", transform.rotation);
     * console.log("Scale:", transform.scale);
     * ```
     */
    export async function transform(entity: number): Promise<Transform> {
        const response = await host.get(host.createUrl(`entities/${entity}/transform`));
        return await response.json();
    }

    /**
     * Updates the transform (position, rotation, scale) of an entity.
     *
     * You can provide a partial transform to update only specific components.
     * For example, you can change just the position while leaving rotation
     * and scale unchanged.
     *
     * @param entity - The entity ID to update
     * @param transform - Partial transform data with the values to update
     *
     * @example
     * ```typescript
     * // Move entity to a new position
     * await entities.setTransform(vrmEntity, {
     *   translation: [0, 100, 0]  // Move up 100 units
     * });
     *
     * // Rotate and scale the entity
     * await entities.setTransform(vrmEntity, {
     *   rotation: [0, 0, 0, 1],   // Reset rotation to identity
     *   scale: [2, 2, 2]          // Double the size
     * });
     *
     * // Update all transform components at once
     * await entities.setTransform(vrmEntity, {
     *   translation: [50, 0, -25],
     *   rotation: [0, 0.707, 0, 0.707],  // 90 degree Y rotation
     *   scale: [1.5, 1.5, 1.5]
     * });
     * ```
     */
    export async function setTransform(
        entity: number,
        transform: Partial<Transform>,
    ): Promise<void> {
        await host.put(host.createUrl(`entities/${entity}/transform`), transform);
    }

    /**
     * Gets the human-readable name of an entity.
     *
     * Most entities in Desktop Homunculus have names that make them easier
     * to identify and work with. VRM models use their character names,
     * bones use standard bone names like "head", "leftHand", etc.
     *
     * @param entity - The entity ID to get the name for
     * @returns A promise that resolves to the entity's name
     *
     * @example
     * ```typescript
     * const vrmEntity = await entities.findByName("MyCharacter");
     * const name = await entities.name(vrmEntity);
     * console.log("Entity name:", name); // "MyCharacter"
     *
     * // Get bone names
     * const headBone = await entities.findByName("head", { root: vrmEntity });
     * const boneName = await entities.name(headBone);
     * console.log("Bone name:", boneName); // "head"
     * ```
     */
    export async function name(entity: number): Promise<string> {
        const response = await host.get(host.createUrl(`entities/${entity}/name`));
        return await response.json();
    }

    /**
     * Finds an entity by its name, optionally within a specific parent entity.
     *
     * This is the primary method for locating entities in the ECS system.
     * Names are unique within their scope (global or under a specific parent).
     *
     * @param name - The name of the entity to find
     * @param options - Optional search parameters
     * @returns A promise that resolves to the entity ID
     * @throws Will throw an error if no entity with the given name is found
     *
     * @example
     * ```typescript
     * // Find a VRM character globally
     * const vrmEntity = await entities.findByName("MyCharacter");
     *
     * // Find a bone within a specific VRM
     * const headBone = await entities.findByName("head", {
     *   root: vrmEntity
     * });
     *
     * // Find UI elements or other named entities
     * const settingsPanel = await entities.findByName("SettingsPanel");
     * const chatWindow = await entities.findByName("ChatWindow");
     * ```
     */
    export async function findByName(
        name: string,
        options?: FindOptions
    ): Promise<number> {
        const response = await host.get(host.createUrl("entities", {
            name,
            ...options,
        }));
        return await response.json();
    }

    export interface MoveTargetWorld {
        type: "world";
        position: Vec2;
        z?: number;
    }

    export interface MoveTargetViewport {
        type: "viewport";
        position: Vec2;
    }

    /**
     * Target position for entity movement.
     *
     * Use `type: "world"` for direct world-space coordinates (position as [x, y], z optional).
     * Use `type: "viewport"` for screen-space coordinates that are automatically converted to world space.
     */
    export type MoveTarget =
        | MoveTargetWorld
        | MoveTargetViewport;

    /**
     * Moves an entity to the specified position.
     *
     * Supports two coordinate types:
     * - **World coordinates**: Sets the entity's position directly in 3D world space.
     *   `z` is optional — if omitted, the entity keeps its current z position.
     * - **Viewport coordinates**: Screen-space position that is automatically converted
     *   to world coordinates internally.
     *
     * @param entity - The entity ID to move
     * @param target - The target position (world or viewport coordinates)
     *
     * @example
     * ```typescript
     * // Move to world coordinates
     * await entities.move(vrmEntity, { type: "world", position: [0, 1.5], z: -2 });
     *
     * // Move to world coordinates (keep current z)
     * await entities.move(vrmEntity, { type: "world", position: [0, 1.5] });
     *
     * // Move to a screen position
     * await entities.move(vrmEntity, { type: "viewport", position: [500, 300] });
     * ```
     */
    export async function move(entity: number, target: MoveTarget): Promise<void> {
        await host.post(host.createUrl(`entities/${entity}/move`), target);
    }

    /**
     * Easing functions for tween animations.
     * Controls the acceleration curve of the animation.
     */
    export type EasingFunction =
        | "linear"
        | "quadraticIn" | "quadraticOut" | "quadraticInOut"
        | "cubicIn" | "cubicOut" | "cubicInOut"
        | "quarticIn" | "quarticOut" | "quarticInOut"
        | "quinticIn" | "quinticOut" | "quinticInOut"
        | "sineIn" | "sineOut" | "sineInOut"
        | "circularIn" | "circularOut" | "circularInOut"
        | "exponentialIn" | "exponentialOut" | "exponentialInOut"
        | "elasticIn" | "elasticOut" | "elasticInOut"
        | "backIn" | "backOut" | "backInOut"
        | "bounceIn" | "bounceOut" | "bounceInOut"
        | "smoothStepIn" | "smoothStepOut" | "smoothStep"
        | "smootherStepIn" | "smootherStepOut" | "smootherStep";

    /**
     * Request parameters for tweening an entity's position.
     */
    export interface TweenPositionRequest {
        /** Target position as [x, y, z] */
        target: [number, number, number];
        /** Duration in milliseconds */
        durationMs: number;
        /** Easing function (default: "linear") */
        easing?: EasingFunction;
        /** Whether to wait for the tween to complete before returning (default: false) */
        wait?: boolean;
    }

    /**
     * Request parameters for tweening an entity's rotation.
     */
    export interface TweenRotationRequest {
        /** Target rotation as quaternion [x, y, z, w] */
        target: [number, number, number, number];
        /** Duration in milliseconds */
        durationMs: number;
        /** Easing function (default: "linear") */
        easing?: EasingFunction;
        /** Whether to wait for the tween to complete before returning (default: false) */
        wait?: boolean;
    }

    /**
     * Request parameters for tweening an entity's scale.
     */
    export interface TweenScaleRequest {
        /** Target scale as [x, y, z] */
        target: [number, number, number];
        /** Duration in milliseconds */
        durationMs: number;
        /** Easing function (default: "linear") */
        easing?: EasingFunction;
        /** Whether to wait for the tween to complete before returning (default: false) */
        wait?: boolean;
    }

    /**
     * Smoothly animate an entity's position to a target value.
     *
     * @param entityId - The entity ID to tween
     * @param request - Tween parameters
     * @returns Promise that resolves when the request completes (or when animation finishes if wait=true)
     *
     * @example
     * ```typescript
     * await entities.tweenPosition(myEntity, {
     *   target: [100, 50, 0],
     *   durationMs: 1000,
     *   easing: "quadraticInOut",
     *   wait: true,
     * });
     * ```
     */
    export async function tweenPosition(entityId: number, request: TweenPositionRequest): Promise<void> {
        await host.post(host.createUrl(`entities/${entityId}/tween/position`), request);
    }

    /**
     * Smoothly animate an entity's rotation to a target value.
     *
     * @param entityId - The entity ID to tween
     * @param request - Tween parameters
     * @returns Promise that resolves when the request completes (or when animation finishes if wait=true)
     *
     * @example
     * ```typescript
     * await entities.tweenRotation(myEntity, {
     *   target: [0, 0, 0.7071, 0.7071], // 90 degrees around Z axis
     *   durationMs: 500,
     *   easing: "elasticOut",
     * });
     * ```
     */
    export async function tweenRotation(entityId: number, request: TweenRotationRequest): Promise<void> {
        await host.post(host.createUrl(`entities/${entityId}/tween/rotation`), request);
    }

    /**
     * Smoothly animate an entity's scale to a target value.
     *
     * @param entityId - The entity ID to tween
     * @param request - Tween parameters
     * @returns Promise that resolves when the request completes (or when animation finishes if wait=true)
     *
     * @example
     * ```typescript
     * await entities.tweenScale(myEntity, {
     *   target: [2, 2, 2],
     *   durationMs: 800,
     *   easing: "bounceOut",
     *   wait: false,
     * });
     * ```
     */
    export async function tweenScale(entityId: number, request: TweenScaleRequest): Promise<void> {
        await host.post(host.createUrl(`entities/${entityId}/tween/scale`), request);
    }
}

import {Vec2, Vec3} from "./math";
import {host} from "./host";

/**
 * Cameras API namespace provides camera utilities, such as coordinate system transformations and viewport management.
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
 * const mousePos: Vec2 = { x: 150, y: 200 };
 * const worldPos2D = await cameras.globalViewportToWorld2d(mousePos);
 * console.log(`Mouse at world 2D:`, worldPos2D);
 *
 * // Convert 3D object position to screen coordinates
 * const vrm = await Vrm.findByName('MyCharacter');
 * const vrmTransform = await entities.transform(vrm.entity);
 * const screenPos = await cameras.worldToGlobalViewport({
 *   x: vrmTransform.translation[0],
 *   y: vrmTransform.translation[1],
 *   z: vrmTransform.translation[2]
 * });
 *
 * const headBone = await vrm.findBoneEntity('head');
 * const headTransform = await entities.transform(headBone);
 * const headScreenPos = await cameras.worldToGlobalViewport({
 *   x: headTransform.translation[0],
 *   y: headTransform.translation[1] + 0.2, // Slightly above head
 *   z: headTransform.translation[2]
 * });
 * ```
 */
export namespace cameras {
    /**
     * Global viewport coordinates representing screen-space positions.
     * These coordinates are relative to the entire desktop/screen area.
     */
    export type GlobalViewport = Vec2;

    /**
     * 2D world coordinates within the 3D scene.
     * Represents positions on a 2D plane within the 3D world space.
     */
    export type World2d = Vec2;

    /**
     * Full 3D world coordinates with x, y, and z components.
     * Represents positions in the complete 3D world space.
     */
    export type World3d = Vec3;

    /**
     * Converts global viewport coordinates to 2D world space coordinates.
     *
     * This transformation maps screen-space coordinates (like mouse positions or
     * UI element positions) into the 2D coordinate system of the 3D world.
     * Useful for placing objects or effects at screen positions within the 3D scene.
     *
     * @param viewport - Screen coordinates to convert (uses center if not provided)
     * @returns A promise that resolves to the corresponding 2D world coordinates
     *
     * @example
     * ```typescript
     * // Convert mouse click to world position
     * document.addEventListener('click', async (event) => {
     *   const worldPos = await cameras.globalViewportToWorld2d({
     *     x: event.clientX,
     *     y: event.clientY
     *   });
     *
     *   console.log(`Clicked at world position:`, worldPos);
     *
     *   // Spawn an effect at the clicked position
     *   await effects.stamp('click-indicator::marker.png', {
     *     position: [worldPos.x, worldPos.y]
     *   });
     * });
     *
     * // Get center of screen in world coordinates
     * const centerWorld = await cameras.globalViewportToWorld2d();
     *
     * // Convert UI element position to world space
     * const buttonElement = document.getElementById('my-button');
     * const buttonRect = buttonElement.getBoundingClientRect();
     * const buttonWorldPos = await cameras.globalViewportToWorld2d({
     *   x: buttonRect.left + buttonRect.width / 2,
     *   y: buttonRect.top + buttonRect.height / 2
     * });
     * ```
     */
    export const globalViewportToWorld2d = async (
        viewport?: Partial<GlobalViewport>
    ): Promise<World2d> => {
        const url = host.createUrl("cameras/world-2d", viewport);
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
     * // Position UI above a VRM character
     * const vrm = await Vrm.findByName('MyCharacter');
     * const vrmTransform = await entities.transform(vrm.entity);
     *
     * const screenPos = await cameras.worldToGlobalViewport({
     *   x: vrmTransform.translation[0],
     *   y: vrmTransform.translation[1] + 1.8, // Above character's head
     *   z: vrmTransform.translation[2]
     * });
     *
     * await Webview.open('character-info', {
     *   position: [screenPos.x - 100, screenPos.y - 50], // Center the 200px wide webview
     *   resolution: [200, 100],
     *   transparent: true
     * });
     *
     * // Track a moving object with screen effects
     * const trackingLoop = async () => {
     *   const currentTransform = await entities.transform(vrm.entity);
     *   const currentScreenPos = await cameras.worldToGlobalViewport({
     *     x: currentTransform.translation[0],
     *     y: currentTransform.translation[1],
     *     z: currentTransform.translation[2]
     *   });
     *
     *   // Update UI element position to follow the VRM
     *   updateFloatingUI(currentScreenPos);
     *
     *   requestAnimationFrame(trackingLoop);
     * };
     * trackingLoop();
     *
     * // Show world origin on screen
     * const originScreenPos = await cameras.worldToGlobalViewport();
     * console.log(`World origin is at screen position:`, originScreenPos);
     *
     * // Position effects relative to bones
     * const leftHand = await vrm.findBoneEntity('leftHand');
     * const handTransform = await entities.transform(leftHand);
     * const handScreenPos = await cameras.worldToGlobalViewport({
     *   x: handTransform.translation[0],
     *   y: handTransform.translation[1],
     *   z: handTransform.translation[2]
     * });
     *
     * await effects.stamp('magic-effect::sparkle.gif', {
     *   bounds: {
     *     min: [handScreenPos.x - 25, handScreenPos.y - 25],
     *     max: [handScreenPos.x + 25, handScreenPos.y + 25]
     *   }
     * });
     * ```
     */
    export const worldToGlobalViewport = async (
        world?: Partial<World3d>
    ): Promise<GlobalViewport> => {
        const url = host.createUrl("cameras/global-view-port", world);
        const response = await host.get(url);
        return await response.json();
    }
}
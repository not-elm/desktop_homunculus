import {host} from "./host";

/**
 * Webview management for creating and controlling embedded web interfaces.
 *
 * Desktop Homunculus uses webviews to provide rich UI experiences that can be
 * positioned anywhere in 3D space or attached to VRM characters. Webviews can
 * display HTML/CSS/JavaScript content from mod assets and provide interactive
 * interfaces for users.
 *
 * Key features:
 * - 3D positioned webviews in world space
 * - VRM-attached webviews that follow characters
 * - Transparent and styled webview windows
 * - Mod asset integration for custom HTML content
 * - Cross-webview communication via commands API
 *
 * @example
 * ```typescript
 * // Open a simple webview at a fixed position
 * const webview = await Webview.open("my-ui-mod", {
 *   position: [100, 200],
 *   resolution: [400, 300],
 *   transparent: true
 * });
 *
 * // Open a webview attached to a VRM character
 * const chatBubble = await Webview.open("chat-bubble", {
 *   caller: vrmEntity,
 *   position: {
 *     vrm: vrmEntity,
 *     bone: "head",
 *     offset: [0, 50],
 *     tracking: true
 *   },
 *   transparent: true,
 *   showToolbar: false
 * });
 *
 * // Check if webview is still open
 * if (!(await webview.isClosed())) {
 *   await webview.close();
 * }
 * ```
 */

/**
 * Configuration options for opening a new webview.
 */
export interface OpenOptions {
    /**
     * The source local path(Relative to the `assets/mods` directory) or
     * URL to display in the webview.
     */
    source: string;
    /**
     * Optional VRM entity ID that will be considered the "caller" of this webview.
     * This creates an association between the webview and a specific VRM character,
     * useful for tracking purposes and enables VRM-specific APIs within the webview.
     */
    caller?: number,

    /**
     * Whether the webview background should be transparent.
     * When true, only the rendered HTML content is visible, allowing for
     * overlay-style UIs and seamless integration with the 3D environment.
     * @defaultValue false
     */
    transparent?: boolean;
    /**
     * Whether to display the webview's toolbar with navigation controls.
     * Disable for immersive UI experiences or when the webview is purely decorative.
     * @defaultValue true
     */
    showToolbar?: boolean,
    /**
     * Whether the webview window should cast a shadow.
     * This visual effect is only applied on macOS platforms.
     * @defaultValue true
     */
    shadow?: boolean,
    /** Position configuration for the webview (screen coordinates or VRM-relative) */
    position?: OpenPosition,
    /**
     * The size of the webview window in pixels [width, height].
     * If not specified, uses Bevy's default webview dimensions.
     * @example [800, 600] for an 800x600 pixel webview
     */
    resolution?: [number, number],

    sounds?: {
        /**
         * Sound to play when the webview is opened.
         * This can be a mod asset sound file path.
         */
        open?: string,
        /**
         * Sound to play when the webview is closed.
         * This can be a mod asset sound file path.
         */
        close?: string,
    }
}

/**
 * Position specification for webview placement.
 * Can be either fixed screen coordinates or relative to a VRM character.
 */
export type OpenPosition = [number, number] | OpenAroundVrm

/**
 * Configuration for positioning a webview relative to a VRM character.
 * Enables dynamic webviews that can follow characters or attach to specific bones.
 */
export interface OpenAroundVrm {
    /** The VRM entity ID to position the webview relative to */
    vrm: number,
    /**
     * Offset from the attachment point in viewport space coordinates [x, y].
     * If not specified, the webview will be positioned at the bone's exact location.
     */
    offset?: [number, number],
    /**
     * Name of the VRM bone to attach to (e.g., "head", "leftHand", "rightFoot").
     * If not specified, attaches to the VRM's root transform.
     */
    bone?: string;
    /**
     * Whether the webview should continuously follow the VRM's movement.
     * When true, the webview will update its position as the VRM moves or animates.
     * @defaultValue false
     */
    tracking?: boolean;
}

/**
 * Represents a webview instance that can display HTML content in 3D space.
 *
 * Webviews are embedded browser windows that can render mod assets and provide
 * interactive user interfaces. They can be positioned freely in 3D space or
 * attached to VRM characters.
 */
export class Webview {
    constructor(readonly entity: number) {
        this.entity = entity;
    }

    /**
     * Closes the webview.
     *
     * @example
     * ```ts
     * await webview.close();
     * ```
     */
    async close(): Promise<void> {
        await host.post(host.createUrl(`webviews/${this.entity}/close`));
    }

    /**
     * Checks whether this webview has been closed.
     *
     * @returns A promise that resolves to true if the webview is closed
     * @example
     * ```typescript
     * if (await webview.isClosed()) {
     *   console.log("Webview was closed");
     * } else {
     *   console.log("Webview is still open");
     * }
     * ```
     */
    async isClosed(): Promise<boolean> {
        const response = await host.get(host.createUrl(`webviews/${this.entity}/is-closed`));
        return await response.json();
    }

    /**
     * Creates and opens a new webview displaying content from a mod asset.
     *
     * @param source - The mod HTML asset source path or URL.
     * @param options - Optional configuration for webview appearance and behavior
     * @returns A promise that resolves to a new Webview instance
     *
     * @example
     * ```typescript
     * // Simple webview with default settings
     * const webview = await Webview.open("my-settings-ui");
     *
     * // Advanced webview attached to VRM character
     * const statusDisplay = await Webview.open("character-status", {
     *   caller: vrmEntity,
     *   position: {
     *     vrm: vrmEntity,
     *     bone: "head",
     *     offset: [0, 100],  // 100 pixels above the head
     *     tracking: true      // Follow the character
     *   },
     *   resolution: [300, 150],
     *   transparent: true,
     *   showToolbar: false,
     *   openSound: "ui-pop"
     * });
     *
     * // Floating UI panel
     * const controlPanel = await Webview.open("control-panel", {
     *   position: [50, 50],     // Top-left corner
     *   resolution: [400, 600],
     *   shadow: true,
     *   closeSound: "ui-close"
     * });
     * ```
     */
    static async open(options?: OpenOptions) {
        const response = await host.post(host.createUrl(`webviews`), options);
        return new Webview(Number(await response.json()));
    }

    /**
     * Gets the current webview instance if called from within a webview context.
     *
     * This static method allows code running inside a webview to get a reference
     * to its own Webview instance for self-management operations.
     *
     * @returns The current Webview instance, or undefined if not in a webview context
     *
     * @example
     * ```typescript
     * // From within a webview's JavaScript code
     * const currentWebview = Webview.current();
     * if (currentWebview) {
     *   // This code is running inside a webview
     *   console.log("Webview entity ID:", currentWebview.entity);
     *
     *   // The webview can close itself
     *   await currentWebview.close();
     * } else {
     *   // This code is running outside of a webview context
     *   console.log("Not running in a webview");
     * }
     * ```
     */
    static current(): Webview | undefined {
        //@ts-ignore
        const entity: number | undefined = window.WEBVIEW_ENTITY;
        return entity !== undefined ? new Webview(entity) : undefined;
    }
}
import {host} from "./host";
import {Rect} from "./math";

/**
 * Effects API namespace for playing visual and audio effects.
 *
 * Provides functionality to trigger various effects that enhance the user experience,
 * including sound effects and visual stamp effects that can be displayed on any monitor.
 * Effects are asset-based, meaning they reference mod assets for their content.
 *
 * Key features:
 * - Sound effect playback from mod assets
 * - Visual stamp effects with customizable positioning and timing
 * - Multi-monitor support for effect placement
 * - Configurable effect parameters (size, duration, bounds)
 *
 * @example
 * ```typescript
 * // Play a sound effect
 * await effects.sound("notification-sounds::ding.wav");
 *
 * // Show a stamp effect at a random position
 * await effects.stamp("reaction-images::heart.png", {
 *   size: [100, 100],
 *   durationSecs: 2.0
 * });
 *
 * // Show stamp effect on a specific display with bounds
 * const displays = await displays.findAll();
 * await effects.stamp("celebrations::confetti.gif", {
 *   display: displays[1].id,  // Second monitor
 *   bounds: {
 *     min: [100, 100],
 *     max: [500, 400]
 *   },
 *   size: [200, 200],
 *   durationSecs: 3.0
 * });
 * ```
 */
export namespace effects {
    /**
     * Configuration options for stamp visual effects.
     * Allows precise control over where and how stamp effects appear on screen.
     */
    export interface StampOptions {
        /**
         * Specify the display/monitor to show the effect on.
         * Use display IDs obtained from `displays.findAll()` to target specific monitors.
         * If not specified, the effect will appear on the primary display.
         */
        display?: number,
        /**
         * Defines the rectangular area where the stamp can appear.
         * The stamp will be randomly positioned within these bounds.
         * Coordinates are in screen pixels relative to the display.
         * If not specified, the stamp can appear anywhere on the display.
         */
        bounds?: Rect,
        /**
         * Size of the stamp effect in pixels [width, height].
         * The original image will be scaled to fit this size.
         * @defaultValue [300, 300]
         */
        size?: [number, number],
        /**
         * How long the stamp effect should remain visible, in seconds.
         * After this time, the stamp will fade out and disappear.
         * @defaultValue 0.8
         */
        durationSecs?: number,
    }

    /**
     * Plays a sound effect from a mod asset.
     *
     * Sound effects are played immediately and do not block execution.
     * The sound file must be included in a mod's assets directory.
     *
     * @param source - The asset path relative to `assets/mods`.
     */
    export const sound = async (source: string) => {
        await host.post(host.createUrl(`effects/sounds`), {
            source,
        });
    }

    /**
     * Displays a visual stamp effect on the screen using an image from mod assets.
     *
     * Stamp effects are temporary visual elements that appear on screen for a short time.
     * They can be used for reactions, notifications, celebrations, or visual feedback.
     * The image will appear at a random position within the specified bounds.
     *
     * @param source - The asset path of the image to use for the stamp effect(relative to `assets/mods`).
     * @param options - Optional configuration for the stamp appearance and behavior
     *
     * @example
     * ```typescript
     * // Simple stamp with default settings
     * await effects.stamp("reactions::thumbs-up.png");
     *
     * // Show effect on secondary monitor
     * const displays = await displays.findAll();
     * const secondMonitor = displays[1];
     *
     * await effects.stamp("notifications/important.png", {
     *   display: secondMonitor.id,
     *   size: [200, 100],
     *   duration_secs: 3.0
     * });
     *
     * // Create a reaction system
     * const reactions = [
     *   "reactions::heart.png",
     *   "reactions::star.png",
     *   "reactions::exclamation.png"
     * ];
     *
     * const randomReaction = reactions[Math.floor(Math.random() * reactions.length)];
     * await effects.stamp(randomReaction, {
     *   size: [80, 80],
     *   duration_secs: 1.5
     * });
     * ```
     */
    export const stamp = async (
        source: string,
        options?: StampOptions,
    ) => {
        await host.post(host.createUrl(`effects/stamps`), {
            source,
            ...options,
        });
    }
}
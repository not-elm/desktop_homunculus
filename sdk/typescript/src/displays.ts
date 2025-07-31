import { Rect } from "./math";
import { host, } from "./host";

/**
 * Displays API namespace for monitor and screen management.
 * 
 * Provides functionality to query information about connected displays/monitors,
 * including their dimensions, positions, and identifiers. This is essential for
 * multi-monitor setups where you need to target specific screens for effects,
 * webviews, or VRM positioning.
 * 
 * Key features:
 * - Discovery of all connected displays
 * - Display identification and metadata
 * - Screen bounds and positioning information
 * - Multi-monitor support for targeting specific screens
 * 
 * @example
 * ```typescript
 * // Get information about all connected displays
 * const allDisplays = await displays.findAll();
 * 
 * console.log(`Found ${allDisplays.length} displays:`);
 * allDisplays.forEach((display, index) => {
 *   console.log(`Display ${index + 1}:`);
 *   console.log(`  ID: ${display.id}`);
 *   console.log(`  Title: ${display.title}`);
 *   console.log(`  Size: ${display.frame.max[0]}x${display.frame.max[1]}`);
 *   console.log(`  Position: (${display.frame.min[0]}, ${display.frame.min[1]})`);
 * });
 * 
 * // Find the primary display (usually the first one)
 * const primaryDisplay = allDisplays[0];
 * 
 * // Target a specific display for effects
 * await effects.stamp("celebration::confetti.gif", {
 *   display: primaryDisplay.id,
 *   bounds: primaryDisplay.frame
 * });
 * 
 * // Open webview on secondary monitor
 * if (allDisplays.length > 1) {
 *   const secondaryDisplay = allDisplays[1];
 *   await Webview.open("dashboard", {
 *     position: [
 *       secondaryDisplay.frame.min[0] + 100,
 *       secondaryDisplay.frame.min[1] + 100
 *     ]
 *   });
 * }
 * ```
 */
export namespace displays {
    /**
     * Represents a connected display/monitor with its properties and dimensions.
     * 
     * Contains all the information needed to identify and target a specific display
     * for positioning elements, effects, or webviews.
     */
    export interface Display {
        /**
         * Unique identifier for this display.
         * Use this ID when targeting specific displays in other API calls.
         */
        id: number;
        
        /**
         * Human-readable name/title of the display.
         * Often includes the manufacturer and model information.
         */
        title: string;
        
        /**
         * The rectangular bounds of the display in global viewport coordinates.
         * - `min`: [x, y] coordinates of the top-left corner
         * - `max`: [x, y] coordinates of the bottom-right corner
         * 
         * For multi-monitor setups, coordinates account for relative positioning
         * between displays.
         */
        frame: Rect;
    }

    /**
     * Retrieves information about all currently connected displays/monitors.
     * 
     * This function queries the system to get real-time information about all
     * available displays, including their positions, sizes, and identifiers.
     * The returned array includes both primary and secondary displays.
     * 
     * @returns A promise that resolves to an array of display information
     * 
     * @example
     * ```typescript
     * // Basic display enumeration
     * const displays = await displays.findAll();
     * console.log(`System has ${displays.length} displays`);
     * 
     * // Display detailed information
     * displays.forEach((display, index) => {
     *   const width = display.frame.max[0] - display.frame.min[0];
     *   const height = display.frame.max[1] - display.frame.min[1];
     *   
     *   console.log(`Display ${index + 1}: ${display.title}`);
     *   console.log(`  Resolution: ${width}x${height}`);
     *   console.log(`  Position: (${display.frame.min[0]}, ${display.frame.min[1]})`);
     * });
     * 
     * // Find the largest display
     * const largestDisplay = displays.reduce((largest, current) => {
     *   const currentArea = 
     *     (current.frame.max[0] - current.frame.min[0]) *
     *     (current.frame.max[1] - current.frame.min[1]);
     *   const largestArea = 
     *     (largest.frame.max[0] - largest.frame.min[0]) *
     *     (largest.frame.max[1] - largest.frame.min[1]);
     *   
     *   return currentArea > largestArea ? current : largest;
     * });
     * 
     * // Use display information for positioning
     * const centerX = (largestDisplay.frame.min[0] + largestDisplay.frame.max[0]) / 2;
     * const centerY = (largestDisplay.frame.min[1] + largestDisplay.frame.max[1]) / 2;
     * 
     * await Webview.open("center-panel", {
     *   position: [centerX - 200, centerY - 150],  // Center a 400x300 window
     *   resolution: [400, 300]
     * });
     * ```
     */
    export const findAll = async (): Promise<Display[]> => {
        const response = await host.get(host.createUrl("displays"));
        return await response.json() as Display[];
    }
}

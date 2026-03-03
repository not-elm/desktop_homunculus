import {type GlobalDisplay} from "./coordinates";
import {host} from "./host";

/**
 * Displays API namespace for monitor and screen management.
 *
 * Provides functionality to query information about connected displays/monitors,
 * including their dimensions, positions, and frame rectangles.
 *
 * @example
 * ```typescript
 * const allDisplays = await displays.findAll();
 * console.log(`Found ${allDisplays.length} displays`);
 * allDisplays.forEach((display) => {
 *   console.log(`${display.title}: (${display.frame.min.x}, ${display.frame.min.y}) - (${display.frame.max.x}, ${display.frame.max.y})`);
 * });
 * ```
 */
export namespace displays {
    /**
     * Retrieves information about all currently connected displays/monitors.
     *
     * @returns A promise that resolves to an array of display information
     *
     * @example
     * ```typescript
     * const allDisplays = await displays.findAll();
     * console.log(`System has ${allDisplays.length} displays`);
     * ```
     */
    export async function findAll(): Promise<GlobalDisplay[]> {
        const response = await host.get(host.createUrl("displays"));
        return await response.json() as GlobalDisplay[];
    }
}

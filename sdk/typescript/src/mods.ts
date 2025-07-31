import {OpenOptions} from "./webviews";
import {host} from "./host";

/**
 * Mods API namespace for managing and interacting with user modifications.
 *
 * Desktop Homunculus supports a powerful mod system that allows users to extend
 * the application with custom HTML, CSS, JavaScript, and assets. Mods can provide
 * new UI interfaces, custom behaviors, and enhanced functionality.
 *
 * The mod system supports:
 * - Custom HTML/CSS/JavaScript content
 * - Asset management (images, sounds, fonts, etc.)
 * - Menu integration for discoverability
 * - Webview-based user interfaces
 * - Cross-mod communication via commands API
 *
 * @example
 * ```typescript
 * // Get all available mod menus
 * const modMenus = await mods.menus();
 *
 * // Display available mods to the user
 * modMenus.forEach(menu => {
 *   console.log(`Mod: ${menu.text}`);
 *   if (menu.thumbnailPath) {
 *     console.log(`Thumbnail: ${menu.thumbnailPath}`);
 *   }
 * });
 *
 * // Open a specific mod's webview
 * const firstMod = modMenus[0];
 * if (firstMod) {
 *   const webview = await Webview.open(
 *     firstMod.webviewAssetId,
 *     firstMod.webviewOptions
 *   );
 * }
 * ```
 */
export namespace mods {
    /**
     * Metadata for a mod menu entry that appears in the application's mod browser.
     *
     * This interface defines the information needed to display and launch a mod
     * from the Desktop Homunculus mod menu system.
     */
    export interface ModMenuMetadata {
        /**
         * Optional path to a thumbnail image for the mod.
         * The path should be relative to the mod's asset directory.
         */
        thumbnail?: string;

        /**
         * Display name for the mod that appears in the menu.
         * This is the human-readable title users will see.
         */
        text: string;

        /**
         * The script local path relative to the mod's asset directory.
         */
        script?: string;

        /**
         * Optional webview configuration for how the mod should be displayed.
         * If not specified, default webview settings will be used.
         */
        webview?: OpenOptions;
    }

    /**
     * Retrieves metadata for all available mod menu entries.
     *
     * This function queries the mod system to get information about all installed
     * mods that have declared menu entries. The returned data can be used to build
     * dynamic mod browsers or selection interfaces.
     *
     * @returns A promise that resolves to an array of mod menu metadata
     *
     * @example
     * ```typescript
     * const modMenus = await mods.menus();
     * ```
     */
    export const menus = async (): Promise<ModMenuMetadata[]> => {
        const response = await host.get(host.createUrl("mods/menus"));
        return await response.json();
    }
}
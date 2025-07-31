import { host } from "./host";

/**
 * Settings API namespace for managing application configuration and preferences.
 * 
 * Provides access to core application settings that affect performance and behavior.
 * Settings are persisted across application restarts and can be used to optimize
 * the application for different hardware configurations or user preferences.
 * 
 * Current settings include:
 * - Frame rate limiting for performance optimization
 * - Future settings for graphics quality, audio levels, etc.
 * 
 * @example
 * ```typescript
 * // Check current FPS limit
 * const currentFps = await settings.fpsLimit();
 * console.log(`Current FPS limit: ${currentFps}`);
 * 
 * // Optimize for high-end hardware
 * await settings.saveFpsLimit(120);
 * 
 * // Optimize for battery life or lower-end hardware
 * await settings.saveFpsLimit(30);
 * 
 * // Remove FPS limit for maximum performance
 * await settings.saveFpsLimit(-1);  // or 0, depending on implementation
 * ```
 */
export namespace settings {
    /**
     * Gets the current frame rate limit setting.
     * 
     * The FPS limit controls how many frames per second the application will render.
     * This is useful for optimizing performance, reducing power consumption, or
     * matching display refresh rates.
     * 
     * @returns A promise that resolves to the current FPS limit
     * 
     * @example
     * ```typescript
     * // Check current setting
     * const fps = await settings.fpsLimit();
     * console.log(`FPS limit: ${fps}`);
     * 
     * // Adjust settings based on current value
     * if (fps < 60) {
     *   console.log("Performance mode is enabled");
     * } else {
     *   console.log("High quality mode is enabled");
     * }
     * 
     * // Use in performance monitoring
     * setInterval(async () => {
     *   const limit = await settings.fpsLimit();
     *   console.log(`Target FPS: ${limit}`);
     * }, 5000);
     * ```
     */
    export const fpsLimit = async (): Promise<number> => {
        const response = await host.get(host.createUrl("settings/fps"));
        return Number(await response.json());
    }

    /**
     * Sets the frame rate limit for the application.
     * 
     * This setting controls the maximum number of frames rendered per second,
     * which directly affects performance, power consumption, and visual smoothness.
     * Changes take effect immediately without requiring an application restart.
     * 
     * @param fps - The maximum frames per second to target
     * 
     * @example
     * ```typescript
     * // Standard settings for different use cases
     * await settings.saveFpsLimit(60);   // Balanced performance
     * await settings.saveFpsLimit(120);  // High refresh rate displays
     * await settings.saveFpsLimit(30);   // Battery saving mode
     * 
     * // Dynamic FPS adjustment based on conditions
     * const isOnBattery = await checkBatteryStatus();
     * const targetFps = isOnBattery ? 30 : 60;
     * await settings.saveFpsLimit(targetFps);
     * 
     * // Performance optimization flow
     * const displays = await displays.findAll();
     * const primaryDisplay = displays[0];
     * const refreshRate = getDisplayRefreshRate(primaryDisplay);
     * 
     * // Match display refresh rate for smooth visuals
     * await settings.saveFpsLimit(refreshRate);
     * 
     * // Adaptive FPS based on application load
     * async function adaptiveFpsControl() {
     *   const vrms = await Vrm.findAll();
     *   
     *   if (vrms.length > 3) {
     *     // Reduce FPS when many VRMs are active
     *     await settings.saveFpsLimit(30);
     *   } else {
     *     // Normal FPS for better visual quality
     *     await settings.saveFpsLimit(60);
     *   }
     * }
     * ```
     */
    export const saveFpsLimit = async (fps: number): Promise<void> => {
        await host.put(host.createUrl("settings/fps"), fps);
    }
}
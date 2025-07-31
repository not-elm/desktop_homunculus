import {host} from "./host";

/**
 * Shadow Panel API namespace for controlling the application's shadow overlay.
 *
 * The shadow panel is a visual overlay that can be used to create an atmospheric
 * effects, focus attention, or provide visual feedback. It appears as
 * a semi-transparent layer over the entire screen, which can be adjusted for
 * transparency to achieve different visual effects.
 *
 * Use cases:
 * - Creating focus modes by dimming the background
 * - Atmospheric effects for different times of day
 * - Visual feedback for application states
 * - Cinematic effects during presentations or demonstrations
 *
 * @example
 * ```typescript
 * // Create a focus mode by dimming the background
 * await shadowPanel.setAlpha(0.7); // 70% opacity shadow
 *
 * // Gradually fade in shadow for dramatic effect
 * for (let alpha = 0; alpha <= 0.8; alpha += 0.1) {
 *   await shadowPanel.setAlpha(alpha);
 *   await sleep(100);
 * }
 *
 * // Check current shadow level
 * const currentAlpha = await shadowPanel.alpha();
 * console.log(`Shadow opacity: ${currentAlpha * 100}%`);
 *
 * // Remove shadow completely
 * await shadowPanel.setAlpha(0);
 *
 * // Create a day/night cycle effect
 * const simulateDayNight = async () => {
 *   const hour = new Date().getHours();
 *
 *   if (hour < 6 || hour > 20) {
 *     // Night time - darker shadow
 *     await shadowPanel.setAlpha(0.6);
 *   } else if (hour < 8 || hour > 18) {
 *     // Dawn/dusk - medium shadow
 *     await shadowPanel.setAlpha(0.3);
 *   } else {
 *     // Day time - no shadow
 *     await shadowPanel.setAlpha(0);
 *   }
 * };
 * ```
 */
export namespace shadowPanel {
    /**
     * Gets the current transparency level of the shadow panel.
     *
     * Returns a value between 0 and 1, where 0 means completely transparent
     * (no shadow visible) and 1 means completely opaque (full shadow).
     *
     * @returns A promise that resolves to the current alpha value (0-1)
     *
     * @example
     * ```typescript
     * // Check current shadow level
     * const alpha = await shadowPanel.alpha();
     * console.log(`Shadow opacity: ${Math.round(alpha * 100)}%`);
     *
     * // Conditional logic based on current shadow
     * const currentAlpha = await shadowPanel.alpha();
     * if (currentAlpha > 0.5) {
     *   console.log('Shadow panel is quite visible');
     * } else if (currentAlpha > 0) {
     *   console.log('Shadow panel is slightly visible');
     * } else {
     *   console.log('Shadow panel is invisible');
     * }
     *
     * // Save current state before changing
     * const savedAlpha = await shadowPanel.alpha();
     * await shadowPanel.setAlpha(0.8); // Temporary change
     * await sleep(2000);
     * await shadowPanel.setAlpha(savedAlpha); // Restore
     *
     * // Monitor shadow changes
     * setInterval(async () => {
     *   const alpha = await shadowPanel.alpha();
     *   document.title = `Shadow: ${Math.round(alpha * 100)}%`;
     * }, 1000);
     * ```
     */
    export const alpha = async () => {
        const response = await host.get(host.createUrl("shadow-panel/alpha"));
        return Number(await response.json());
    }

    /**
     * Sets the transparency level of the shadow panel.
     *
     * Controls how opaque the shadow overlay appears. A value of 0 makes the
     * shadow completely invisible, while 1 makes it completely opaque. Values
     * in between create varying levels of transparency for different effects.
     *
     * @param alpha - The transparency value between 0 (invisible) and 1 (opaque)
     *
     * @example
     * ```typescript
     * // Basic shadow control
     * await shadowPanel.setAlpha(0);    // No shadow (fully transparent)
     * await shadowPanel.setAlpha(0.3);  // Light shadow
     * await shadowPanel.setAlpha(0.7);  // Heavy shadow
     * await shadowPanel.setAlpha(1);    // Full shadow (completely opaque)
     *
     * // Create a fade-in effect
     * const fadeInShadow = async (targetAlpha: number, duration: number) => {
     *   const steps = 20;
     *   const stepDuration = duration / steps;
     *
     *   for (let i = 0; i <= steps; i++) {
     *     const alpha = (targetAlpha * i) / steps;
     *     await shadowPanel.setAlpha(alpha);
     *     await sleep(stepDuration);
     *   }
     * };
     *
     * await fadeInShadow(0.6, 2000); // Fade to 60% over 2 seconds
     *
     * // Create a breathing effect
     * const breathingEffect = async () => {
     *   for (let i = 0; i < 10; i++) {
     *     await shadowPanel.setAlpha(0.2 + 0.3 * Math.sin(i * 0.5));
     *     await sleep(500);
     *   }
     * };
     *
     * // Attention-grabbing pulse
     * const pulseAttention = async () => {
     *   const originalAlpha = await shadowPanel.alpha();
     *
     *   for (let i = 0; i < 3; i++) {
     *     await shadowPanel.setAlpha(0.8);
     *     await sleep(200);
     *     await shadowPanel.setAlpha(originalAlpha);
     *     await sleep(200);
     *   }
     * };
     *
     * // Responsive shadow based on time
     * const timeBasedShadow = async () => {
     *   const hour = new Date().getHours();
     *   const nightTime = hour < 7 || hour > 19;
     *
     *   await shadowPanel.setAlpha(nightTime ? 0.4 : 0.1);
     * };
     * ```
     */
    export const setAlpha = async (alpha: number): Promise<void> => {
        await host.put(host.createUrl("shadow-panel/alpha"), {alpha});
    }
}
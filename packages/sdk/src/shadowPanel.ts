import { host } from './host';

/** Request body for setting shadow panel alpha. */
export interface ShadowPanelPutBody {
  alpha: number;
}

/**
 * Shadow Panel API namespace for controlling the application's shadow overlay.
 *
 * The shadow panel is a visual overlay that can be used to create atmospheric
 * effects, focus attention, or provide visual feedback.
 *
 * @example
 * ```typescript
 * await shadowPanel.setAlpha(0.7);
 * const currentAlpha = await shadowPanel.alpha();
 * await shadowPanel.setAlpha(0);
 * ```
 */
export namespace shadowPanel {
  /**
   * Gets the current transparency level of the shadow panel.
   *
   * @returns A promise that resolves to the current alpha value (0-1)
   */
  export async function alpha() {
    const response = await host.get(host.createUrl('shadow-panel/alpha'));
    return Number(await response.json());
  }

  /**
   * Sets the transparency level of the shadow panel.
   *
   * @param alpha - The transparency value between 0 (invisible) and 1 (opaque)
   *
   * @example
   * ```typescript
   * await shadowPanel.setAlpha(0.7);
   * ```
   */
  export async function setAlpha(alpha: number): Promise<void> {
    await host.put(host.createUrl('shadow-panel/alpha'), {
      alpha,
    });
  }
}

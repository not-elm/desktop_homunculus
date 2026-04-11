import { host } from './host';

/** Request body for setting frame rate. */
export interface SetFpsBody {
  fps: number;
}

/**
 * Settings API namespace for controlling application-level configuration.
 *
 * @example
 * ```typescript
 * const currentFps = await settings.fps();
 * await settings.setFps(30);
 * ```
 */
export namespace settings {
  /**
   * Gets the current frame rate (FPS).
   *
   * @returns A promise that resolves to the current FPS value
   *
   * @example
   * ```typescript
   * const fps = await settings.fps();
   * console.log(`Current FPS: ${fps}`);
   * ```
   */
  export async function fps(): Promise<number> {
    const response = await host.get(host.createUrl('settings/fps'));
    return Number(await response.json());
  }

  /**
   * Sets the frame rate (FPS). Persists and applies immediately.
   *
   * @param fps - The target frame rate in frames per second (1-120)
   *
   * @example
   * ```typescript
   * await settings.setFps(30);
   * ```
   */
  export async function setFps(fps: number): Promise<void> {
    await host.put(host.createUrl('settings/fps'), { fps });
  }
}

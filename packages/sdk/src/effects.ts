import { host } from './host';

/** Request body for creating a stamp effect. */
export interface StampRequestBody {
  asset: string;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  alpha?: number;
  duration?: number;
}

/**
 * Effects API namespace for visual effects.
 *
 * Provides functionality to trigger visual stamp effects that enhance the user experience.
 *
 * For audio playback, see the {@link audio} namespace.
 *
 * @example
 * ```typescript
 * // Show a stamp effect
 * await effects.stamp("heart-reaction", {
 *   width: 100,
 *   height: 100,
 *   duration: 2.0
 * });
 * ```
 */
export namespace effects {
  /**
   * Configuration options for stamp visual effects.
   */
  export interface StampOptions {
    /** X position on screen. */
    x?: number;
    /** Y position on screen. */
    y?: number;
    /** Width in pixels. */
    width?: number;
    /** Height in pixels. */
    height?: number;
    /** Opacity (0-1). */
    alpha?: number;
    /** Duration in seconds. */
    duration?: number;
  }

  /**
   * Displays a visual stamp effect on the screen.
   *
   * @param asset - The asset ID of the stamp image.
   * @param options - Optional configuration for the stamp appearance
   *
   * @example
   * ```typescript
   * await effects.stamp("thumbs-up");
   *
   * await effects.stamp("heart", {
   *   x: 100,
   *   y: 200,
   *   width: 80,
   *   height: 80,
   *   duration: 1.5
   * });
   * ```
   */
  export async function stamp(asset: string, options?: StampOptions) {
    await host.post(host.createUrl(`effects/stamps`), {
      asset,
      ...options,
    });
  }
}

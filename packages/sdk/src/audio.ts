import { host } from './host';

/**
 * Audio API namespace for playing sound effects and background music.
 *
 * Provides functionality for one-shot sound effects ({@link audio.se}) and
 * looping background music with transport controls ({@link audio.bgm}).
 *
 * @example
 * ```typescript
 * // Play a sound effect
 * await audio.se.play("click");
 *
 * // Play background music with fade-in
 * await audio.bgm.play("heme", {
 *   volume: 0.8,
 *   fadeIn: { durationSecs: 2.0 }
 * });
 *
 * // Stop BGM with fade-out
 * await audio.bgm.stop({ fadeOut: { durationSecs: 1.5 } });
 * ```
 */
export namespace audio {
  /**
   * Configuration options for sound effect playback.
   */
  export interface SeOptions {
    /** Volume level (0.0-1.0). Default: 1.0 */
    volume?: number;
    /** Playback speed. Default: 1.0 */
    speed?: number;
    /** Stereo panning (-1.0 left to 1.0 right). Default: 0.0 */
    panning?: number;
  }

  /**
   * Tween configuration for fade transitions.
   */
  export interface FadeTween {
    /** Duration in seconds */
    durationSecs: number;
    /** Easing function. Default: "linear" */
    easing?: 'linear' | 'easeIn' | 'easeOut' | 'easeInOut';
  }

  /**
   * Configuration options for starting BGM playback.
   */
  export interface BgmPlayOptions {
    /** Loop playback. Default: true */
    loop?: boolean;
    /** Volume level (0.0-1.0). Default: 1.0 */
    volume?: number;
    /** Playback speed. Default: 1.0 */
    speed?: number;
    /** Fade-in settings */
    fadeIn?: FadeTween;
  }

  /**
   * Configuration options for stopping BGM playback.
   */
  export interface BgmStopOptions {
    /** Fade-out settings. Omit for immediate stop */
    fadeOut?: FadeTween;
  }

  /**
   * Configuration options for updating BGM playback parameters.
   */
  export interface BgmUpdateOptions {
    /** New volume level */
    volume?: number;
    /** New playback speed */
    speed?: number;
    /** Transition settings */
    tween?: FadeTween;
  }

  /**
   * Current BGM playback status.
   */
  export interface BgmStatus {
    /** Current asset ID (null if stopped) */
    asset: string | null;
    /** Playback state */
    state: 'playing' | 'paused' | 'stopped';
    /** Loop setting */
    loop: boolean;
    /** Current volume */
    volume: number;
    /** Current speed */
    speed: number;
  }

  /**
   * Sound effects (SE) sub-namespace for one-shot audio playback.
   *
   * @example
   * ```typescript
   * // Simple sound effect
   * await audio.se.play("my-mod:notification");
   *
   * // With options
   * await audio.se.play("my-mod:alert", {
   *   volume: 0.5,
   *   speed: 1.2,
   *   panning: -0.5
   * });
   * ```
   */
  export namespace se {
    /**
     * Plays a one-shot sound effect.
     *
     * @param asset - The asset ID of the sound effect (e.g., `"click"`)
     * @param options - Optional playback configuration
     *
     * @example
     * ```typescript
     * await audio.se.play("click");
     *
     * await audio.se.play("coin", {
     *   volume: 0.7,
     *   speed: 1.5,
     *   panning: 0.3
     * });
     * ```
     */
    export async function play(asset: string, options?: SeOptions) {
      await host.post(host.createUrl('audio/se'), { asset, ...options });
    }
  }

  /**
   * Background music (BGM) sub-namespace for continuous audio playback with transport controls.
   *
   * Only one BGM track plays at a time. Starting a new track replaces the current one.
   *
   * @example
   * ```typescript
   * // Play looping background music
   * await audio.bgm.play("theme");
   *
   * // Pause and resume
   * await audio.bgm.pause();
   * await audio.bgm.resume();
   *
   * // Check current status
   * const status = await audio.bgm.status();
   * console.log(status.state); // "playing" | "paused" | "stopped"
   * ```
   */
  export namespace bgm {
    /**
     * Plays background music, replacing any currently playing BGM.
     *
     * @param asset - The asset ID of the music track (e.g., `"theme"`)
     * @param options - Optional playback configuration
     *
     * @example
     * ```typescript
     * // Simple playback (loops by default)
     * await audio.bgm.play("my-mod:battle");
     *
     * // With options
     * await audio.bgm.play("my-mod:intro", {
     *   loop: false,
     *   volume: 0.6,
     *   fadeIn: { durationSecs: 3.0, easing: "easeIn" }
     * });
     * ```
     */
    export async function play(asset: string, options?: BgmPlayOptions) {
      await host.post(host.createUrl('audio/bgm'), { asset, ...options });
    }

    /**
     * Stops the currently playing BGM.
     *
     * @param options - Optional stop configuration (e.g., fade-out)
     *
     * @example
     * ```typescript
     * // Immediate stop
     * await audio.bgm.stop();
     *
     * // Fade out over 2 seconds
     * await audio.bgm.stop({
     *   fadeOut: { durationSecs: 2.0, easing: "easeOut" }
     * });
     * ```
     */
    export async function stop(options?: BgmStopOptions) {
      await host.post(host.createUrl('audio/bgm/stop'), { ...options });
    }

    /**
     * Pauses the currently playing BGM.
     *
     * @example
     * ```typescript
     * await audio.bgm.pause();
     * ```
     */
    export async function pause() {
      await host.post(host.createUrl('audio/bgm/pause'), {});
    }

    /**
     * Resumes paused BGM playback.
     *
     * @example
     * ```typescript
     * await audio.bgm.resume();
     * ```
     */
    export async function resume() {
      await host.post(host.createUrl('audio/bgm/resume'), {});
    }

    /**
     * Updates playback parameters of the currently playing BGM.
     *
     * @param options - The parameters to update
     *
     * @example
     * ```typescript
     * // Fade volume to 0.3 over 1 second
     * await audio.bgm.update({
     *   volume: 0.3,
     *   tween: { durationSecs: 1.0, easing: "easeInOut" }
     * });
     *
     * // Change speed immediately
     * await audio.bgm.update({ speed: 0.8 });
     * ```
     */
    export async function update(options: BgmUpdateOptions) {
      await host.patch(host.createUrl('audio/bgm'), options);
    }

    /**
     * Gets the current BGM playback status.
     *
     * @returns The current BGM status including asset, state, volume, and speed
     *
     * @example
     * ```typescript
     * const status = await audio.bgm.status();
     * if (status.state === "playing") {
     *   console.log(`Now playing: ${status.asset} at volume ${status.volume}`);
     * }
     * ```
     */
    export async function status(): Promise<BgmStatus> {
      const response = await host.get(host.createUrl('audio/bgm'));
      return (await response.json()) as BgmStatus;
    }
  }
}

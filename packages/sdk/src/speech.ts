/**
 * A single keyframe in a speech timeline.
 */
export interface TimelineKeyframe {
    /**
     * Duration of this keyframe in seconds.
     */
    duration: number;
    /**
     * Expression targets to set during this keyframe.
     * Keys are expression names (e.g. "aa", "ih", "happy"), values are weights (0.0-1.0).
     */
    targets?: Record<string, number>;
}

/**
 * Speech utilities for converting phoneme data to Timeline keyframes.
 *
 * @example
 * ```typescript
 * import { speech, persona } from "@hmcs/sdk";
 *
 * const keyframes = speech.fromPhonemes([
 *   ["aa", 0.1],
 *   [null, 0.05],
 *   ["oh", 0.12],
 * ]);
 * const p = await persona.load("alice");
 * await p.vrm().speakWithTimeline(wavData, keyframes);
 * ```
 */
export namespace speech {

    /**
     * Creates timeline keyframes from a simple phoneme list.
     *
     * Each entry is a tuple of [expression_name | null, duration_seconds].
     * A null expression name creates a silent keyframe.
     *
     * @param phonemes - Array of [expression_name, duration] tuples.
     * @returns An array of timeline keyframes.
     *
     * @example
     * ```typescript
     * const keyframes = speech.fromPhonemes([
     *   ["aa", 0.1],
     *   [null, 0.05],
     *   ["oh", 0.12],
     * ]);
     * ```
     */
    export function fromPhonemes(phonemes: Array<[string | null, number]>): TimelineKeyframe[] {
        return phonemes.map(([name, duration]) => {
            if (name) {
                return {duration, targets: {[name]: 1.0}};
            }
            return {duration};
        });
    }
}

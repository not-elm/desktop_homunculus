import type { TimelineKeyframe } from "@hmcs/sdk";

interface VoiceVoxMora {
  vowel: string;
  vowel_length: number;
  consonant_length?: number;
}

interface VoiceVoxAccentPhrase {
  moras: VoiceVoxMora[];
  pause_mora?: VoiceVoxMora;
}

interface VoiceVoxAudioQuery {
  accent_phrases: VoiceVoxAccentPhrase[];
  speedScale: number;
  prePhonemeLength: number;
  postPhonemeLength: number;
}

const VOWEL_MAP: Record<string, string> = {
  a: "aa", b: "aa", h: "aa", l: "aa", m: "aa", p: "aa",
  i: "ih", d: "ih", f: "ih", n: "ih", r: "ih", t: "ih", v: "ih",
  u: "ou",
  e: "ee", j: "ee", s: "ee", x: "ee", y: "ee", z: "ee",
  o: "oh", g: "oh", q: "oh", w: "oh",
};

/**
 * Converts a VoiceVox AudioQuery into Timeline keyframes.
 */
export function voicevoxToTimeline(query: VoiceVoxAudioQuery): TimelineKeyframe[] {
  const keyframes: TimelineKeyframe[] = [];

  // Pre-phoneme silence
  keyframes.push({ duration: query.prePhonemeLength / query.speedScale });

  for (const phrase of query.accent_phrases) {
    for (const mora of phrase.moras) {
      const duration = (mora.vowel_length + (mora.consonant_length ?? 0)) / query.speedScale;
      const expression = VOWEL_MAP[mora.vowel];
      if (expression) {
        keyframes.push({ duration, targets: { [expression]: 1.0 } });
      } else {
        keyframes.push({ duration });
      }
    }
    if (phrase.pause_mora) {
      const pm = phrase.pause_mora;
      const duration = pm.vowel_length / query.speedScale;
      const expression = VOWEL_MAP[pm.vowel];
      if (expression) {
        keyframes.push({ duration, targets: { [expression]: 1.0 } });
      } else {
        keyframes.push({ duration });
      }
    }
  }

  // Post-phoneme silence
  keyframes.push({ duration: query.postPhonemeLength / query.speedScale });

  return keyframes;
}

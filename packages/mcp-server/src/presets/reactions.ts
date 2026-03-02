export interface ReactionPreset {
  expressions: Record<string, number>;
  vrma: string | null;
  se: string | null;
}

export const REACTION_PRESETS: Record<string, ReactionPreset> = {
  happy: {
    expressions: { happy: 1.0 },
    vrma: "idle-happy",
    se: "success",
  },
  sad: {
    expressions: { sad: 0.8 },
    vrma: null,
    se: null,
  },
  confused: {
    expressions: { surprised: 0.4 },
    vrma: null,
    se: null,
  },
  error: {
    expressions: { angry: 0.3, sad: 0.4 },
    vrma: null,
    se: "error",
  },
  success: {
    expressions: { happy: 0.9 },
    vrma: "celebrate",
    se: "success",
  },
  thinking: {
    expressions: { neutral: 0.5 },
    vrma: "thinking",
    se: null,
  },
  surprised: {
    expressions: { surprised: 0.9 },
    vrma: null,
    se: "notification",
  },
  neutral: {
    expressions: {},
    vrma: null,
    se: null,
  },
};

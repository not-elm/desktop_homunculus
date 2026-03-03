import { describe, it, expect } from "vitest";
import { REACTION_PRESETS, type ReactionPreset } from "../presets/reactions.js";

describe("REACTION_PRESETS", () => {
  const requiredReactions = [
    "happy", "sad", "confused", "error",
    "success", "thinking", "surprised", "neutral",
  ];

  it("should have all required reaction presets", () => {
    for (const name of requiredReactions) {
      expect(REACTION_PRESETS[name]).toBeDefined();
    }
  });

  it("each preset should have expressions object", () => {
    for (const [name, preset] of Object.entries(REACTION_PRESETS)) {
      expect(preset.expressions).toBeDefined();
      expect(typeof preset.expressions).toBe("object");
    }
  });

  it("neutral should have empty expressions (clear all)", () => {
    expect(REACTION_PRESETS.neutral.expressions).toEqual({});
  });

  it("happy should set happy expression", () => {
    expect(REACTION_PRESETS.happy.expressions.happy).toBeGreaterThan(0);
  });
});

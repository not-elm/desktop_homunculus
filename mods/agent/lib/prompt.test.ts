import { describe, it, expect } from "vitest";
import { buildCharacterPrompt } from "./prompt.ts";
import type { Persona } from "./types.ts";

const basePersona: Persona = {
  name: "TestChar",
  age: 25,
  gender: "female",
  firstPersonPronoun: "watashi",
  profile: "",
  personality: null,
};

describe("buildCharacterPrompt", () => {
  describe("basic persona fields", () => {
    it("includes character name", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain('"TestChar"');
    });

    it("includes age", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("Age: 25");
    });

    it("shows Unknown when age is null", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, age: null });
      expect(prompt).toContain("Age: Unknown");
    });

    it("includes gender label", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("Gender: Female");
    });

    it("includes first-person pronoun instruction", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain('"watashi"');
    });

    it("omits pronoun line when null", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        firstPersonPronoun: null,
      });
      expect(prompt).not.toContain("first-person pronoun");
    });
  });

  describe("profile", () => {
    it("includes profile when non-empty", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        profile: "A cheerful girl",
      });
      expect(prompt).toContain("Profile: A cheerful girl");
    });

    it("omits profile when empty string", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, profile: "" });
      expect(prompt).not.toContain("Profile:");
    });
  });

  describe("personality section", () => {
    it("omits section when personality is null", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, personality: null });
      expect(prompt).not.toContain("## Personality");
    });

    it("omits section when personality is empty string", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, personality: "" });
      expect(prompt).not.toContain("## Personality");
    });

    it("omits section when personality is undefined", () => {
      const { personality: _, ...withoutPersonality } = basePersona;
      const prompt = buildCharacterPrompt(withoutPersonality as Persona);
      expect(prompt).not.toContain("## Personality");
    });

    it("includes personality text under ## Personality header", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        personality: "Sarcastic but caring",
      });
      expect(prompt).toContain("## Personality");
      expect(prompt).toContain("Sarcastic but caring");
    });

    it("places personality before response style section", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        personality: "Cheerful and bright",
      });
      const personalityIdx = prompt.indexOf("## Personality");
      const styleIdx = prompt.indexOf("## Response Style");
      expect(personalityIdx).toBeLessThan(styleIdx);
    });

    it("places profile before personality", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        profile: "An apprentice wizard",
        personality: "Curious and talkative",
      });
      const profileIdx = prompt.indexOf("Profile:");
      const personalityIdx = prompt.indexOf("## Personality");
      expect(profileIdx).toBeLessThan(personalityIdx);
    });
  });

  describe("existing sections", () => {
    it("includes response style section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("## Response Style");
    });

    it("includes response examples section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("## Response Examples");
    });

    it("includes webview section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("open_webview");
    });
  });
});

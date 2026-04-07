import { describe, it, expect } from "vitest";
import { buildCharacterPrompt } from "./prompt.ts";
import type { Persona } from "./types.ts";

const basePersona: Persona = {
  name: "テスト",
  age: 25,
  gender: "female",
  firstPersonPronoun: "わたし",
  profile: "",
  personality: null,
};

describe("buildCharacterPrompt", () => {
  describe("basic persona fields", () => {
    it("includes character name", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("「テスト」");
    });

    it("includes age", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("年齢: 25歳");
    });

    it("shows 不詳 when age is null", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, age: null });
      expect(prompt).toContain("年齢: 不詳");
    });

    it("includes gender label", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("性別: 女性");
    });

    it("includes first-person pronoun instruction", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("「わたし」");
    });

    it("omits pronoun line when null", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        firstPersonPronoun: null,
      });
      expect(prompt).not.toContain("一人称");
    });
  });

  describe("profile", () => {
    it("includes profile when non-empty", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        profile: "明るくて元気な女の子",
      });
      expect(prompt).toContain("プロフィール: 明るくて元気な女の子");
    });

    it("omits profile when empty string", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, profile: "" });
      expect(prompt).not.toContain("プロフィール:");
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
        personality: "皮肉っぽいが思いやりがある",
      });
      expect(prompt).toContain("## Personality");
      expect(prompt).toContain("皮肉っぽいが思いやりがある");
    });

    it("places personality before spoken style section", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        personality: "元気で明るい",
      });
      const personalityIdx = prompt.indexOf("## Personality");
      const styleIdx = prompt.indexOf("## 応答スタイル");
      expect(personalityIdx).toBeLessThan(styleIdx);
    });

    it("places profile before personality", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        profile: "魔法使いの見習い",
        personality: "好奇心旺盛で話好き",
      });
      const profileIdx = prompt.indexOf("プロフィール:");
      const personalityIdx = prompt.indexOf("## Personality");
      expect(profileIdx).toBeLessThan(personalityIdx);
    });
  });

  describe("existing sections", () => {
    it("includes spoken style section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("## 応答スタイル");
    });

    it("includes few-shot section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("## 応答の例");
    });

    it("includes webview section", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).toContain("open_webview");
    });
  });
});

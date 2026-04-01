import { describe, it, expect } from "vitest";
import { buildCharacterPrompt } from "./prompt.ts";
import type { Persona } from "./types.ts";

const basePersona: Persona = {
  name: "テスト",
  age: 25,
  gender: "female",
  firstPersonPronoun: "わたし",
  profile: "",
  ocean: {},
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

  describe("OCEAN section", () => {
    it("omits section when ocean is empty object", () => {
      const prompt = buildCharacterPrompt({ ...basePersona, ocean: {} });
      expect(prompt).not.toContain("話し方の傾向");
    });

    it("omits section when all traits are neutral", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: {
          openness: 0.5,
          conscientiousness: 0.5,
          extraversion: 0.5,
          agreeableness: 0.5,
          neuroticism: 0.5,
        },
      });
      expect(prompt).not.toContain("話し方の傾向");
    });

    it("omits section when traits are at neutral boundaries", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.35, neuroticism: 0.65 },
      });
      expect(prompt).not.toContain("話し方の傾向");
    });

    it("includes section when a trait is high (>0.65)", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.8 },
      });
      expect(prompt).toContain("## 話し方の傾向");
    });

    it("includes section when a trait is low (<0.35)", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.1 },
      });
      expect(prompt).toContain("## 話し方の傾向");
    });

    it("contains integration framing sentence", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.9 },
      });
      expect(prompt).toContain("一貫した人物像として表現してください");
    });

    it("high extraversion produces descriptors with よ/ね", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.9 },
      });
      expect(prompt).toMatch(/[よね]/);
      expect(prompt).toContain("積極的に話題を広げる");
    });

    it("low extraversion produces descriptors with かな/けど", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.1 },
      });
      expect(prompt).toContain("かな");
      expect(prompt).toContain("けど");
    });

    it("high neuroticism produces hedging markers", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { neuroticism: 0.9 },
      });
      expect(prompt).toContain("かもしれない");
    });

    it("low neuroticism produces calm assertive descriptor", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { neuroticism: 0.1 },
      });
      expect(prompt).toContain("断定的な語調");
    });

    it("places profile before OCEAN before spoken style", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        profile: "魔法使いの見習い",
        ocean: { extraversion: 0.9 },
      });
      const profileIdx = prompt.indexOf("プロフィール:");
      const oceanIdx = prompt.indexOf("## 話し方の傾向");
      const styleIdx = prompt.indexOf("## 応答スタイル");
      expect(profileIdx).toBeLessThan(oceanIdx);
      expect(oceanIdx).toBeLessThan(styleIdx);
    });

    it.each([
      ["extraversion", 0.1, "かな"],
      ["extraversion", 0.9, "よ"],
      ["agreeableness", 0.1, "別に"],
      ["agreeableness", 0.9, "そうですよね"],
      ["neuroticism", 0.1, "断定的"],
      ["neuroticism", 0.9, "かもしれない"],
      ["openness", 0.1, "実用的"],
      ["openness", 0.9, "好奇心"],
      ["conscientiousness", 0.1, "自由に話し"],
      ["conscientiousness", 0.9, "順序立てて"],
    ])("%s=%s produces descriptor containing '%s'", (trait, value, expected) => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { [trait]: value },
      });
      expect(prompt).toContain(expected);
    });

    it("multiple non-neutral traits produce multiple descriptors", () => {
      const prompt = buildCharacterPrompt({
        ...basePersona,
        ocean: { extraversion: 0.9, neuroticism: 0.9, openness: 0.1 },
      });
      const oceanSection = prompt.split("## 話し方の傾向")[1]?.split("##")[0] ?? "";
      const descriptorLines = oceanSection
        .split("\n")
        .filter((line) => line.startsWith("- "));
      expect(descriptorLines.length).toBe(3);
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

  describe("removed features", () => {
    it("does not contain 性格: label", () => {
      const prompt = buildCharacterPrompt(basePersona);
      expect(prompt).not.toContain("性格:");
    });
  });
});

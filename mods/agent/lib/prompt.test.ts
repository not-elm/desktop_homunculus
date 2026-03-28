import { describe, it, expect } from "vitest";
import { buildCharacterPrompt } from "./prompt.ts";

describe("buildCharacterPrompt", () => {
  const persona = { name: "テスト", personality: "元気で明るい" };

  it("includes character name", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("「テスト」");
  });

  it("includes personality instruction", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("元気で明るい");
  });

  it("includes few-shot bad example", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("悪い例");
  });

  it("includes few-shot good example", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("良い例");
  });

  it("includes positive instruction about weaving in supplements", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("織り込");
  });

  it("includes symbol replacement guidance for plus sign", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("と");
    // Should mention avoiding + or ＋
    expect(prompt).toMatch(/[+＋]/);
  });

  it("includes bracket avoidance instruction", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("括弧");
  });

  it("includes webview delegation instruction", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("open_webview");
  });

  it("omits personality when empty string", () => {
    const prompt = buildCharacterPrompt({ name: "テスト", personality: "" });
    expect(prompt).not.toContain("性格:");
  });

  it("includes personality priority note", () => {
    const prompt = buildCharacterPrompt(persona);
    expect(prompt).toContain("口語体");
  });
});

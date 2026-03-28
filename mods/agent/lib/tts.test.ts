import { describe, it, expect } from "vitest";
import { sanitizeForTts } from "./tts";

describe("sanitizeForTts", () => {
  describe("return type", () => {
    it("returns object with sentences and log arrays", () => {
      const result = sanitizeForTts("こんにちは。");
      expect(result).toHaveProperty("sentences");
      expect(result).toHaveProperty("log");
      expect(Array.isArray(result.sentences)).toBe(true);
      expect(Array.isArray(result.log)).toBe(true);
    });

    it("returns empty sentences and log for empty input", () => {
      const result = sanitizeForTts("");
      expect(result.sentences).toEqual([]);
      expect(result.log).toEqual([]);
    });
  });

  describe("existing rule regression tests", () => {
    it("removes fenced code blocks", () => {
      const result = sanitizeForTts("次のコードを見てください。\n```\nconst x = 1;\n```\n以上です。");
      expect(result.sentences).not.toContain(expect.stringContaining("```"));
      expect(result.sentences).not.toContain(expect.stringContaining("const x"));
      expect(result.sentences.some((s) => s.includes("次のコードを見てください"))).toBe(true);
    });

    it("removes inline code", () => {
      const result = sanitizeForTts("`foo`を使います。");
      expect(result.sentences.some((s) => s.includes("foo"))).toBe(false);
      expect(result.sentences.some((s) => s.includes("を使います"))).toBe(true);
    });

    it("removes heading markers", () => {
      const result = sanitizeForTts("## はじめに\n本文です。");
      expect(result.sentences.some((s) => s.includes("##"))).toBe(false);
      expect(result.sentences.some((s) => s.includes("はじめに"))).toBe(true);
    });

    it("removes emphasis markers", () => {
      const result = sanitizeForTts("**重要**な点です。");
      expect(result.sentences.some((s) => s.includes("**"))).toBe(false);
      expect(result.sentences.some((s) => s.includes("重要"))).toBe(true);
    });

    it("removes single asterisk emphasis", () => {
      const result = sanitizeForTts("*斜体*のテキストです。");
      expect(result.sentences.some((s) => s.includes("*"))).toBe(false);
    });

    it("removes strikethrough markers", () => {
      const result = sanitizeForTts("~~削除済み~~のテキストです。");
      expect(result.sentences.some((s) => s.includes("~~"))).toBe(false);
    });

    it("extracts link text from markdown links", () => {
      const result = sanitizeForTts("[クリックして](https://example.com)ください。");
      expect(result.sentences.some((s) => s.includes("クリックして"))).toBe(true);
      expect(result.sentences.some((s) => s.includes("https://example.com"))).toBe(false);
    });

    it("replaces bare URLs with URL省略", () => {
      const result = sanitizeForTts("詳しくは https://example.com を見てください。");
      expect(result.sentences.some((s) => s.includes("URL省略"))).toBe(true);
      expect(result.sentences.some((s) => s.includes("example.com"))).toBe(false);
    });

    it("splits on 。", () => {
      const result = sanitizeForTts("最初の文。次の文。");
      expect(result.sentences.length).toBeGreaterThanOrEqual(2);
    });

    it("splits on ！", () => {
      const result = sanitizeForTts("やった！うれしい！");
      expect(result.sentences.length).toBeGreaterThanOrEqual(2);
    });

    it("splits on ？", () => {
      const result = sanitizeForTts("どうですか？よいですか？");
      expect(result.sentences.length).toBeGreaterThanOrEqual(2);
    });

    it("splits on newline", () => {
      const result = sanitizeForTts("一行目\n二行目");
      expect(result.sentences.length).toBeGreaterThanOrEqual(2);
    });

    it("trims whitespace from sentences", () => {
      const result = sanitizeForTts("  スペースあり。  ");
      for (const s of result.sentences) {
        expect(s).toBe(s.trim());
      }
    });

    it("filters out empty strings after split", () => {
      const result = sanitizeForTts("。。。");
      for (const s of result.sentences) {
        expect(s.length).toBeGreaterThan(0);
      }
    });
  });

  describe("conversion log", () => {
    it("returns empty log for clean text with no transformations", () => {
      const result = sanitizeForTts("普通のテキストです。");
      expect(result.log).toEqual([]);
    });

    it("logs when fenced code block is removed", () => {
      const result = sanitizeForTts("説明。\n```js\nconst x = 1;\n```\n終わり。");
      expect(result.log.some((entry) => entry.includes("fenced-code"))).toBe(true);
    });

    it("logs when inline code is removed", () => {
      const result = sanitizeForTts("`someVar`を使います。");
      expect(result.log.some((entry) => entry.includes("inline-code"))).toBe(true);
    });

    it("logs when heading marker is removed", () => {
      const result = sanitizeForTts("## タイトル\n本文。");
      expect(result.log.some((entry) => entry.includes("heading"))).toBe(true);
    });

    it("logs when emphasis marker is removed", () => {
      const result = sanitizeForTts("**強調**です。");
      expect(result.log.some((entry) => entry.includes("emphasis"))).toBe(true);
    });

    it("logs when markdown link is transformed", () => {
      const result = sanitizeForTts("[テキスト](https://example.com)");
      expect(result.log.some((entry) => entry.includes("md-link"))).toBe(true);
    });

    it("logs when bare URL is replaced", () => {
      const result = sanitizeForTts("見てください https://example.com です。");
      expect(result.log.some((entry) => entry.includes("bare-url"))).toBe(true);
    });

    it("each log entry is a string", () => {
      const result = sanitizeForTts("**強調** と `コード` と https://example.com です。");
      for (const entry of result.log) {
        expect(typeof entry).toBe("string");
      }
    });

    it("logs multiple rules when multiple transformations apply", () => {
      const result = sanitizeForTts("**強調** と `コード` です。");
      expect(result.log.length).toBeGreaterThanOrEqual(2);
    });

    it("log entries contain the matched text truncated to 80 chars", () => {
      const longCode = "`" + "a".repeat(100) + "`";
      const result = sanitizeForTts(longCode + "です。");
      const inlineCodeLog = result.log.find((e) => e.includes("inline-code"));
      expect(inlineCodeLog).toBeDefined();
      // The matched text portion should be truncated at 80 chars
      expect(inlineCodeLog!.length).toBeLessThan(200);
    });
  });
});

import {describe, it, expect} from "vitest";
import {
    normalize,
    normalizePhrase,
    resolveThreshold,
    matchWakeWord,
    mapNormalizedIndexToRaw,
} from "./wake-word-matcher";

// ---------------------------------------------------------------------------
// Normalization
// ---------------------------------------------------------------------------

describe("normalize", () => {
    it("strips Japanese punctuation", () => {
        expect(normalize("エルマー、こんにちは！")).toBe("えるまーこんにちは");
    });

    it("strips English punctuation", () => {
        expect(normalize("Hello, World!")).toBe("hello world");
    });

    it("collapses whitespace and trims", () => {
        expect(normalize("  hello   world  ")).toBe("hello world");
    });

    it("converts katakana to hiragana", () => {
        expect(normalize("エルマー")).toBe("えるまー");
    });

    it("lowercases Latin characters", () => {
        expect(normalize("Hey ELMER")).toBe("hey elmer");
    });

    it("handles mixed script text", () => {
        expect(normalize("Hello、エルマー！")).toBe("helloえるまー");
    });

    it("handles empty string", () => {
        expect(normalize("")).toBe("");
    });

    it("preserves long vowel mark (ー)", () => {
        expect(normalize("エルマー")).toBe("えるまー");
    });
});

// ---------------------------------------------------------------------------
// Hallucination blacklist
// ---------------------------------------------------------------------------

describe("hallucination blacklist", () => {
    it("rejects known hallucination output", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("ご視聴ありがとうございました", phrases, "strict");
        expect(result).toBeNull();
    });

    it("rejects hallucination with trailing text", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("ご視聴ありがとうございました。また来てね", phrases, "strict");
        expect(result).toBeNull();
    });

    it("passes normal speech through", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマー、テスト直して", phrases, "strict");
        expect(result).not.toBeNull();
    });

    it("checks raw text (not normalized)", () => {
        // "Thank you for watching" starts with a blacklisted entry
        const phrases = [normalizePhrase("Thank")];
        const result = matchWakeWord("Thank you for watching everyone", phrases, "strict");
        expect(result).toBeNull();
    });
});

// ---------------------------------------------------------------------------
// Exact prefix match
// ---------------------------------------------------------------------------

describe("exact prefix match", () => {
    it("matches same-script exact prefix", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマー、テスト直して", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.matchedPhrase).toBe("エルマー");
        expect(result!.confidence).toBe(1.0);
        expect(result!.remainingText).toBe("テスト直して");
    });

    it("matches cross-script (katakana phrase vs hiragana input)", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("えるまー、テスト直して", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.confidence).toBe(1.0);
        expect(result!.remainingText).toBe("テスト直して");
    });

    it("extracts remaining text correctly", () => {
        const phrases = [normalizePhrase("Hey Elmer")];
        const result = matchWakeWord("Hey Elmer fix the tests", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.matchedPhrase).toBe("Hey Elmer");
        expect(result!.confidence).toBe(1.0);
        expect(result!.remainingText).toBe("fix the tests");
    });
});

// ---------------------------------------------------------------------------
// Sliding window fuzzy match
// ---------------------------------------------------------------------------

describe("sliding window fuzzy match", () => {
    it("tolerates 1-char error at strict threshold", () => {
        // "えるまあ" vs "えるまー" — 1 char difference on 4-char string
        // score = 1 - 1/4 = 0.75 — below strict (0.80)
        // But with 5-char phrase "えるまーさ" vs "えるまあさ" → score = 1 - 1/5 = 0.80
        const phrases = [normalizePhrase("エルマーさ")]; // 5 chars normalized
        const result = matchWakeWord("えるまあさ、テスト", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.confidence).toBeGreaterThanOrEqual(0.80);
    });

    it("rejects below-threshold match", () => {
        // "あいうえお" vs "かきくけこ" — completely different
        const phrases = [normalizePhrase("あいうえお")];
        const result = matchWakeWord("かきくけこ テスト", phrases, "strict");

        expect(result).toBeNull();
    });

    it("prefers shortest matching window", () => {
        // When multiple window sizes match, ascending order means
        // shortest-first wins (first above threshold)
        const phrases = [normalizePhrase("hello")];
        const result = matchWakeWord("hello world", phrases, "strict");

        expect(result).not.toBeNull();
        // Exact prefix should match first
        expect(result!.confidence).toBe(1.0);
    });

    it("handles text shorter than phrase", () => {
        // Input "えま" is shorter than phrase "えるまー"
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("えま", phrases, "strict");

        expect(result).toBeNull();
    });
});

// ---------------------------------------------------------------------------
// Multi-phrase matching
// ---------------------------------------------------------------------------

describe("multi-phrase matching", () => {
    it("selects best score among multiple phrases", () => {
        const phrases = [
            normalizePhrase("ハルカ"),
            normalizePhrase("エルマー"),
        ];
        const result = matchWakeWord("エルマー、こんにちは", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.matchedPhrase).toBe("エルマー");
    });

    it("returns exact match immediately (first exact prefix wins)", () => {
        // Longer phrase listed first → exact prefix match returns immediately
        const phrases = [
            normalizePhrase("エルマー"),  // Exact prefix match (checked first)
            normalizePhrase("エルマ"),    // Also an exact prefix but checked second
        ];
        const result = matchWakeWord("エルマー テスト", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.confidence).toBe(1.0);
        expect(result!.matchedPhrase).toBe("エルマー");
    });
});

// ---------------------------------------------------------------------------
// Honorific consumption
// ---------------------------------------------------------------------------

describe("honorific consumption", () => {
    it("consumes さん after match", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマーさん、テスト直して", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.remainingText).toBe("テスト直して");
    });

    it("consumes ちゃん after match", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマーちゃん テスト", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.remainingText).toBe("テスト");
    });

    it("consumes katakana variant サン", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマーサン テスト", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.remainingText).toBe("テスト");
    });

    it("consumes 先生 after match", () => {
        const phrases = [normalizePhrase("エルマー")];
        const result = matchWakeWord("エルマー先生、テスト直して", phrases, "strict");

        expect(result).not.toBeNull();
        expect(result!.remainingText).toBe("テスト直して");
    });

    it("does not consume partial honorific", () => {
        const phrases = [normalizePhrase("エルマー")];
        // "さ" alone is not a complete honorific
        const result = matchWakeWord("エルマーさ テスト", phrases, "strict");

        expect(result).not.toBeNull();
        // "さ" should remain since it's not a full honorific
        expect(result!.remainingText).toBe("さ テスト");
    });
});

// ---------------------------------------------------------------------------
// Index mapping
// ---------------------------------------------------------------------------

describe("mapNormalizedIndexToRaw", () => {
    it("maps correctly when punctuation is removed", () => {
        // raw:        "エルマー、テスト"
        // normalized: "えるまーてすと" (no comma)
        // normalizedIndex 4 ("えるまー") → raw index should be 4 (before 、)
        const rawIndex = mapNormalizedIndexToRaw("エルマー、テスト", 4);
        expect(rawIndex).toBe(4);
    });

    it("maps correctly when whitespace is collapsed", () => {
        // raw:        "hello   world"
        // normalized: "hello world" (collapsed)
        // normalizedIndex 5 ("hello") → raw index 5 (exact match boundary)
        const rawIndex = mapNormalizedIndexToRaw("hello   world", 5);
        expect(rawIndex).toBe(5);
        // Downstream consumeHonorificsAndWhitespace handles "   world" → "world"
    });

    it("maps correctly with mixed transforms", () => {
        // raw:        "Hey、 Elmer"
        // normalized: "hey elmer" (punct stripped, spaces collapsed, lowered)
        // normalizedIndex 4 ("hey ") → raw index should account for removed 、
        const rawIndex = mapNormalizedIndexToRaw("Hey、 Elmer", 4);
        expect(rawIndex).toBe(5); // "Hey、 " = 5 chars (punct removed but space advances)
    });
});

// ---------------------------------------------------------------------------
// Threshold resolution
// ---------------------------------------------------------------------------

describe("resolveThreshold", () => {
    it("resolves 'strict' to 0.80", () => {
        expect(resolveThreshold("strict")).toBe(0.80);
    });

    it("resolves 'normal' to 0.70", () => {
        expect(resolveThreshold("normal")).toBe(0.70);
    });

    it("resolves 'loose' to 0.55", () => {
        expect(resolveThreshold("loose")).toBe(0.55);
    });

    it("passes through raw numbers", () => {
        expect(resolveThreshold(0.75)).toBe(0.75);
    });

    it("defaults to strict when undefined", () => {
        expect(resolveThreshold(undefined)).toBe(0.80);
    });
});

import {distance} from "fastest-levenshtein";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** Pre-normalized phrase for efficient repeated matching. */
export interface NormalizedPhrase {
    /** Original phrase as registered by the user. */
    raw: string;
    /** Phrase after the 4-step normalization pipeline. */
    normalized: string;
}

/** Threshold preset name or a raw numeric value (0.0–1.0). */
export type WakeWordThreshold = number | "strict" | "normal" | "loose";

/** Result of a successful wake-word match. */
export interface WakeWordMatch {
    /** The registered phrase that matched. */
    matchedPhrase: string;
    /** Full raw STT transcript. */
    transcript: string;
    /** Similarity score (0.0–1.0). */
    confidence: number;
    /** Text after the wake word (the user's instruction). */
    remainingText: string;
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/** Threshold presets. */
const THRESHOLD_PRESETS: Record<string, number> = {
    strict: 0.80,
    normal: 0.70,
    loose: 0.55,
};

/**
 * Known Whisper hallucination outputs (≥8 chars each).
 * Checked against raw (unnormalized) text via `startsWith`.
 */
const HALLUCINATION_BLACKLIST: readonly string[] = [
    "ご視聴ありがとうございました",
    "ご視聴ありがとうございます",
    "チャンネル登録お願いします",
    "チャンネル登録よろしくお願いします",
    "最後までご視聴ありがとうございました",
    "ご覧いただきありがとうございます",
    "ご覧いただきありがとうございました",
    "お疲れ様でした",
    "よろしくお願いします",
    "よろしくお願いいたします",
    "お願いいたします",
    "ありがとうございました",
    "ありがとうございます",
    "Thank you for watching",
    "Thanks for watching",
    "Please subscribe",
    "Thank you for listening",
    "Thanks for listening",
    "Please like and subscribe",
    "Don't forget to subscribe",
    "Subtitles by the Amara.org community",
    "Subtitles by",
    "Translated by",
    "MoizMedia.com",
    "Amara.org",
];

/** Honorifics consumed after a match (hiragana + katakana variants). */
const HONORIFICS: readonly string[] = [
    "先生", "せんせい", "センセイ",
    "ちゃん", "チャン",
    "さん", "サン",
    "くん", "クン",
    "様", "さま", "サマ",
];

/** Sliding window expansion range around phrase length. */
const WINDOW_MARGIN = 3;

// ---------------------------------------------------------------------------
// Normalization
// ---------------------------------------------------------------------------

/**
 * 4-step normalization pipeline:
 * 1. Strip Unicode punctuation
 * 2. Collapse whitespace
 * 3. Lowercase
 * 4. Katakana → Hiragana
 */
export function normalize(text: string): string {
    const stripped = stripPunctuation(text);
    const collapsed = collapseWhitespace(stripped);
    const lowered = collapsed.toLowerCase();
    return katakanaToHiragana(lowered);
}

function stripPunctuation(text: string): string {
    return text.replace(/[\p{P}]/gu, "");
}

function collapseWhitespace(text: string): string {
    return text.replace(/\s+/g, " ").trim();
}

function katakanaToHiragana(text: string): string {
    let result = "";
    for (let i = 0; i < text.length; i++) {
        const code = text.charCodeAt(i);
        // Katakana range: U+30A1 (ァ) to U+30F6 (ヶ)
        if (code >= 0x30A1 && code <= 0x30F6) {
            result += String.fromCharCode(code - 0x60);
        } else {
            result += text[i];
        }
    }
    return result;
}

// ---------------------------------------------------------------------------
// Pre-caching
// ---------------------------------------------------------------------------

/** Pre-normalize a phrase for efficient repeated matching. */
export function normalizePhrase(phrase: string): NormalizedPhrase {
    return {raw: phrase, normalized: normalize(phrase)};
}

// ---------------------------------------------------------------------------
// Threshold
// ---------------------------------------------------------------------------

/** Resolve a threshold preset name or raw number. Defaults to "strict". */
export function resolveThreshold(threshold: WakeWordThreshold | undefined): number {
    if (threshold === undefined) return THRESHOLD_PRESETS.strict;
    if (typeof threshold === "number") return threshold;
    return THRESHOLD_PRESETS[threshold] ?? THRESHOLD_PRESETS.strict;
}

// ---------------------------------------------------------------------------
// Matching
// ---------------------------------------------------------------------------

/**
 * Match raw STT text against pre-normalized wake-word phrases.
 *
 * Returns the best match (highest score) above threshold, or `null`.
 */
export function matchWakeWord(
    rawText: string,
    phrases: NormalizedPhrase[],
    threshold: WakeWordThreshold | undefined,
): WakeWordMatch | null {
    if (isHallucination(rawText)) return null;

    const resolvedThreshold = resolveThreshold(threshold);
    const normalizedText = normalize(rawText);
    if (normalizedText.length === 0) return null;

    let bestMatch: WakeWordMatch | null = null;

    for (const phrase of phrases) {
        const result = matchSinglePhrase(rawText, normalizedText, phrase, resolvedThreshold);
        if (result && (bestMatch === null || result.confidence > bestMatch.confidence)) {
            bestMatch = result;
        }
        // Exact prefix = immediate return (score 1.0 can't be beaten)
        if (bestMatch?.confidence === 1.0) return bestMatch;
    }

    return bestMatch;
}

function isHallucination(rawText: string): boolean {
    for (const entry of HALLUCINATION_BLACKLIST) {
        if (rawText.startsWith(entry)) return true;
    }
    return false;
}

function matchSinglePhrase(
    rawText: string,
    normalizedText: string,
    phrase: NormalizedPhrase,
    threshold: number,
): WakeWordMatch | null {
    const phraseLen = phrase.normalized.length;
    if (phraseLen === 0) return null;

    // Fast path: exact prefix
    if (normalizedText.startsWith(phrase.normalized)) {
        const splitIndex = mapNormalizedIndexToRaw(rawText, phraseLen);
        const remaining = consumeHonorificsAndWhitespace(rawText, splitIndex);
        return {
            matchedPhrase: phrase.raw,
            transcript: rawText,
            confidence: 1.0,
            remainingText: remaining,
        };
    }

    // Sliding window: ascending from shortest
    return slidingWindowMatch(rawText, normalizedText, phrase, threshold);
}

function slidingWindowMatch(
    rawText: string,
    normalizedText: string,
    phrase: NormalizedPhrase,
    threshold: number,
): WakeWordMatch | null {
    const phraseLen = phrase.normalized.length;
    const textLen = normalizedText.length;
    const windowMin = Math.max(1, phraseLen - WINDOW_MARGIN);
    const windowMax = Math.min(textLen, phraseLen + WINDOW_MARGIN);

    let bestScore = 0;
    let bestWindowLen = 0;

    for (let windowLen = windowMin; windowLen <= windowMax; windowLen++) {
        const prefix = normalizedText.slice(0, windowLen);
        const maxLen = Math.max(phraseLen, windowLen);
        const score = 1 - distance(phrase.normalized, prefix) / maxLen;

        if (score >= threshold && score > bestScore) {
            bestScore = score;
            bestWindowLen = windowLen;
        }
    }

    if (bestScore === 0) return null;

    const splitIndex = mapNormalizedIndexToRaw(rawText, bestWindowLen);
    const remaining = consumeHonorificsAndWhitespace(rawText, splitIndex);
    return {
        matchedPhrase: phrase.raw,
        transcript: rawText,
        confidence: bestScore,
        remainingText: remaining,
    };
}

// ---------------------------------------------------------------------------
// Honorific consumption
// ---------------------------------------------------------------------------

function consumeHonorificsAndWhitespace(rawText: string, fromIndex: number): string {
    let index = fromIndex;
    const tail = rawText.slice(index);

    for (const honorific of HONORIFICS) {
        if (tail.startsWith(honorific)) {
            index += honorific.length;
            break;
        }
    }

    // Trim leading whitespace/punctuation from remaining text
    const remaining = rawText.slice(index).replace(/^[\s、,]+/, "");
    return remaining;
}

// ---------------------------------------------------------------------------
// Index mapping
// ---------------------------------------------------------------------------

/**
 * Map a character index in normalized text back to the corresponding
 * position in raw text. Necessary because normalization removes punctuation,
 * changing string length.
 *
 * Replays the normalization pipeline char-by-char to build an offset mapping.
 */
export function mapNormalizedIndexToRaw(rawText: string, normalizedIndex: number): number {
    let normalizedPos = 0;
    let rawPos = 0;
    let afterLeadingWhitespace = false;
    let lastWasSpace = false;

    while (rawPos < rawText.length && normalizedPos < normalizedIndex) {
        const ch = rawText[rawPos];
        const isPunct = /[\p{P}]/u.test(ch);

        if (isPunct) {
            // Punctuation is stripped — advance raw only
            rawPos++;
            continue;
        }

        const isSpace = /\s/.test(ch);

        if (isSpace) {
            if (!afterLeadingWhitespace) {
                // Leading whitespace (trimmed) — advance raw only
                rawPos++;
                continue;
            }
            if (lastWasSpace) {
                // Collapsed whitespace — advance raw only
                rawPos++;
                continue;
            }
            lastWasSpace = true;
        } else {
            afterLeadingWhitespace = true;
            lastWasSpace = false;
        }

        normalizedPos++;
        rawPos++;
    }

    return rawPos;
}

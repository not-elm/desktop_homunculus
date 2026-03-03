#!/usr/bin/env -S node --experimental-strip-types

import { z } from "zod";
import { input as commandInput, StdinParseError } from "@hmcs/sdk/commands";
import { fail, fetchWithTimeout } from "../lib/utils.ts";

const FETCH_TIMEOUT_MS = 30_000;
const DEFAULT_SPEAKER = 0;
const DEFAULT_VOICEVOX_HOST = "http://localhost:50021";

const schema = z.object({
  speaker: z.number().default(DEFAULT_SPEAKER),
  voicevox_host: z.string().default(DEFAULT_VOICEVOX_HOST),
});

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

const defaults = { speaker: DEFAULT_SPEAKER, voicevox_host: DEFAULT_VOICEVOX_HOST };
let parsed = defaults;
try {
  parsed = await commandInput.parse(schema) as typeof parsed;
} catch (err) {
  if (err instanceof StdinParseError && err.code === "EMPTY_STDIN") {
    // Use defaults (already set by initializer)
  } else if (err instanceof StdinParseError) {
    fail("INVALID_INPUT", err.message, 2);
  } else {
    throw err;
  }
}

const speaker = parsed.speaker;
const voicevoxHost = parsed.voicevox_host.replace(/\/+$/, "");

// --- Initialize speaker on VoiceVox ---

let response: Response;
try {
  response = await fetchWithTimeout(
    `${voicevoxHost}/initialize_speaker?speaker=${speaker}`,
    { method: "POST" },
    FETCH_TIMEOUT_MS,
  );
} catch (err: unknown) {
  const error = err as Error;
  if (error.name === "AbortError") {
    fail("VOICEVOX_UNREACHABLE", `VoiceVox /initialize_speaker timed out after ${FETCH_TIMEOUT_MS}ms`, 1);
  }
  fail("VOICEVOX_UNREACHABLE", `Cannot reach VoiceVox at ${voicevoxHost}: ${error.message}`, 1);
}

if (!response.ok) {
  const body = await response.text().catch(() => "");
  fail("INITIALIZE_FAILED", `/initialize_speaker returned ${response.status}: ${body}`, 1);
}

// --- Success output ---
console.log(JSON.stringify({ success: true, speaker }));

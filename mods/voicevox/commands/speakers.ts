#!/usr/bin/env tsx

import { z } from "zod";
import { input as commandInput, output, StdinParseError } from "@hmcs/sdk/commands";
import { fetchWithTimeout } from "../lib/utils.ts";

const FETCH_TIMEOUT_MS = 10_000;
const DEFAULT_VOICEVOX_HOST = "http://localhost:50021";

const schema = z.object({
  voicevox_host: z.string().default(DEFAULT_VOICEVOX_HOST),
});

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

const defaults = { voicevox_host: DEFAULT_VOICEVOX_HOST };
let parsed = defaults;
try {
  parsed = await commandInput.parse(schema) as typeof parsed;
} catch (err) {
  if (err instanceof StdinParseError) {
    // Silently ignore malformed optional input; use defaults (already set by initializer).
  } else {
    throw err;
  }
}

const voicevoxHost = parsed.voicevox_host.replace(/\/+$/, "");

// --- Fetch speakers from VoiceVox ---

let response: Response;
try {
  response = await fetchWithTimeout(
    `${voicevoxHost}/speakers`,
    { method: "GET" },
    FETCH_TIMEOUT_MS,
  );
} catch (err: unknown) {
  const error = err as Error;
  if (error.name === "AbortError") {
    output.fail("VOICEVOX_UNREACHABLE", `VoiceVox /speakers timed out after ${FETCH_TIMEOUT_MS}ms`);
  }
  output.fail("VOICEVOX_UNREACHABLE", `Cannot reach VoiceVox at ${voicevoxHost}: ${error.message}`);
}

if (!response.ok) {
  const body = await response.text().catch(() => "");
  output.fail("SPEAKERS_FAILED", `/speakers returned ${response.status}: ${body}`);
}

const speakers = await response.json();

// --- Success output ---
output.succeed(speakers);

#!/usr/bin/env -S node --experimental-strip-types

import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import { input as commandInput, StdinParseError } from "@hmcs/sdk/commands";
import { voicevoxToTimeline } from "../lib/timeline.ts";
import { fail, fetchWithTimeout } from "../lib/utils.ts";

const DEFAULT_SPEAKER = 0;
const DEFAULT_VOICEVOX_HOST = "http://localhost:50021";
const DEFAULT_FETCH_TIMEOUT_MS = 30_000;

const schema = z.object({
  entity: z.number(),
  text: z.union([z.string(), z.array(z.string()).min(1)]),
  speaker: z.number().default(DEFAULT_SPEAKER),
  voicevox_host: z.string().default(DEFAULT_VOICEVOX_HOST),
  speed_scale: z.number().optional(),
  pitch_scale: z.number().optional(),
  intonation_scale: z.number().optional(),
  volume_scale: z.number().optional(),
  fetch_timeout_ms: z.number().default(DEFAULT_FETCH_TIMEOUT_MS),
});

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

type Input = z.infer<typeof schema>;
let parsed!: Input;
try {
  parsed = await commandInput.parse(schema) as Input;
} catch (err) {
  if (err instanceof StdinParseError) {
    fail("INVALID_INPUT", err.message, 2);
  }
  throw err;
}

const sentences = Array.isArray(parsed.text) ? parsed.text : [parsed.text];
const speaker = parsed.speaker;
const voicevoxHost = parsed.voicevox_host.replace(/\/+$/, "");
const fetchTimeoutMs = parsed.fetch_timeout_ms;

const vrm = new Vrm(parsed.entity);

// --- Process each sentence sequentially ---

for (const sentence of sentences) {
  // Step 1: Request AudioQuery from VoiceVox
  let aqResponse: Response;
  try {
    const url = `${voicevoxHost}/audio_query?speaker=${speaker}&text=${encodeURIComponent(sentence)}`;
    aqResponse = await fetchWithTimeout(url, { method: "POST" }, fetchTimeoutMs);
  } catch (err: unknown) {
    const error = err as Error;
    if (error.name === "AbortError") {
      fail("VOICEVOX_UNREACHABLE", `VoiceVox audio_query timed out after ${fetchTimeoutMs}ms`, 1);
    }
    fail("VOICEVOX_UNREACHABLE", `Cannot reach VoiceVox at ${voicevoxHost}: ${error.message}`, 1);
  }

  if (!aqResponse.ok) {
    const body = await aqResponse.text().catch(() => "");
    fail("AUDIO_QUERY_FAILED", `audio_query returned ${aqResponse.status}: ${body}`, 1);
  }

  const query = await aqResponse.json();

  // Step 2: Apply scale overrides BEFORE generating keyframes
  if (parsed.speed_scale != null) {
    query.speedScale = parsed.speed_scale;
  }
  if (parsed.pitch_scale != null) {
    query.pitchScale = parsed.pitch_scale;
  }
  if (parsed.intonation_scale != null) {
    query.intonationScale = parsed.intonation_scale;
  }
  if (parsed.volume_scale != null) {
    query.volumeScale = parsed.volume_scale;
  }

  // Step 3: Generate keyframes from the MODIFIED query
  let keyframes;
  try {
    keyframes = voicevoxToTimeline(query);
  } catch (err: unknown) {
    const error = err as Error;
    fail("TIMELINE_FAILED", `Failed to generate timeline: ${error.message}`, 1);
  }

  // Step 4: Synthesise audio with the SAME modified query
  let synthResponse: Response;
  try {
    const url = `${voicevoxHost}/synthesis?speaker=${speaker}`;
    synthResponse = await fetchWithTimeout(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(query),
    }, fetchTimeoutMs);
  } catch (err: unknown) {
    const error = err as Error;
    if (error.name === "AbortError") {
      fail("VOICEVOX_UNREACHABLE", `VoiceVox synthesis timed out after ${fetchTimeoutMs}ms`, 1);
    }
    fail("VOICEVOX_UNREACHABLE", `Cannot reach VoiceVox at ${voicevoxHost}: ${error.message}`, 1);
  }

  if (!synthResponse.ok) {
    const body = await synthResponse.text().catch(() => "");
    fail("SYNTHESIS_FAILED", `synthesis returned ${synthResponse.status}: ${body}`, 1);
  }

  const wav = await synthResponse.arrayBuffer();

  // Step 5: Play audio with lip-sync timeline
  try {
    console.log(`Playing sentence: "`, keyframes);
    await vrm.speakWithTimeline(wav, keyframes, { waitForCompletion: true });
  } catch (err: unknown) {
    const msg = (err as Error).message ?? String(err);
    if (/not.found/i.test(msg) || /404/i.test(msg)) {
      fail("ENTITY_NOT_FOUND", `Entity ${parsed.entity} not found: ${msg}`, 1);
    }
    fail("SYNTHESIS_FAILED", `speakWithTimeline failed: ${msg}`, 1);
  }
}

// --- Success output ---
console.log(JSON.stringify({
  success: true,
  sentences: sentences.length,
  speaker,
}));

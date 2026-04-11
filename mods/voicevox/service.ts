import { Persona, type PersonaVrm, preferences } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import { z } from 'zod';
import { voicevoxToTimeline } from './lib/timeline.ts';
import { fetchWithTimeout } from './lib/utils.ts';

const VOICEVOX_HOST = 'http://localhost:50021';
const FETCH_TIMEOUT_MS = 30_000;

const DEFAULTS = {
  speakerId: 0,
  speedScale: 1.0,
  pitchScale: 0.0,
  intonationScale: 1.0,
  volumeScale: 1.0,
  pauseLength: 1.0,
  prePhonemeLength: 0.1,
  postPhonemeLength: 0.1,
};

interface VoicevoxSettings {
  speakerId: number;
  speedScale: number;
  pitchScale: number;
  intonationScale: number;
  volumeScale: number;
  pauseLength: number;
  prePhonemeLength: number;
  postPhonemeLength: number;
}

const characterLocks = new Map<string, Promise<unknown>>();

function withCharacterLock<T>(personaId: string, fn: () => Promise<T>): Promise<T> {
  const prev = characterLocks.get(personaId) ?? Promise.resolve();
  const result = prev.then(fn, fn);
  characterLocks.set(personaId, result);
  result.finally(() => {
    if (characterLocks.get(personaId) === result) characterLocks.delete(personaId);
  });
  return result;
}

const speakerInitPromises = new Map<number, Promise<void>>();
const initializedSpeakers = new Set<number>();

async function ensureSpeakerInitialized(speakerId: number): Promise<void> {
  if (initializedSpeakers.has(speakerId)) return;

  const existing = speakerInitPromises.get(speakerId);
  if (existing) return existing;

  const promise = warmupSpeaker(speakerId)
    .then(() => {
      initializedSpeakers.add(speakerId);
    })
    .catch((err) => {
      console.error(`Speaker ${speakerId} init failed:`, err);
    })
    .finally(() => {
      speakerInitPromises.delete(speakerId);
    });

  speakerInitPromises.set(speakerId, promise);
  return promise;
}

async function warmupSpeaker(speakerId: number): Promise<void> {
  const url = `${VOICEVOX_HOST}/initialize_speaker?speaker=${speakerId}`;
  const response = await fetchWithTimeout(url, { method: 'POST' }, FETCH_TIMEOUT_MS);
  if (!response.ok) {
    throw new Error(`/initialize_speaker returned ${response.status}`);
  }
}

function clearInitializedSpeakers(): void {
  initializedSpeakers.clear();
}

async function resolveAssetId(personaId: string): Promise<string | null> {
  const p = await Persona.load(personaId);
  const snapshot = await p.snapshot();
  return snapshot.vrmAssetId ?? null;
}

async function loadSettings(assetId: string | null): Promise<VoicevoxSettings> {
  if (!assetId) return { ...DEFAULTS };
  const saved = await preferences.load<VoicevoxSettings>(`voicevox::${assetId}`);
  return saved ? { ...DEFAULTS, ...saved } : { ...DEFAULTS };
}

async function speakSentence(
  vrm: PersonaVrm,
  sentence: string,
  settings: VoicevoxSettings,
): Promise<void> {
  const audioQuery = await fetchAudioQuery(sentence, settings.speakerId);
  applyVoiceParams(audioQuery, settings);
  const keyframes = generateTimeline(audioQuery);
  const wav = await synthesize(audioQuery, settings.speakerId);
  try {
    await vrm.speakWithTimeline(wav, keyframes, { waitForCompletion: true });
  } catch (err) {
    throw new Error(`speakWithTimeline failed: ${(err as Error).message}`);
  }
}

async function fetchAudioQuery(sentence: string, speakerId: number): Promise<any> {
  const url = `${VOICEVOX_HOST}/audio_query?speaker=${speakerId}&text=${encodeURIComponent(sentence)}`;
  let response: Response;
  try {
    response = await fetchWithTimeout(url, { method: 'POST' }, FETCH_TIMEOUT_MS);
  } catch (err) {
    clearInitializedSpeakers();
    const error = err as Error;
    throw new Error(`VoiceVox unreachable at ${VOICEVOX_HOST}: ${error.message}`);
  }
  if (!response.ok) {
    const body = await response.text().catch(() => '');
    throw new Error(`audio_query failed (${response.status}): ${body}`);
  }
  return response.json();
}

function applyVoiceParams(query: any, settings: VoicevoxSettings): void {
  query.speedScale = settings.speedScale;
  query.pitchScale = settings.pitchScale;
  query.intonationScale = settings.intonationScale;
  query.volumeScale = settings.volumeScale;
  query.pauseLength = settings.pauseLength;
  query.prePhonemeLength = settings.prePhonemeLength;
  query.postPhonemeLength = settings.postPhonemeLength;
}

function generateTimeline(query: any) {
  try {
    return voicevoxToTimeline(query);
  } catch (err) {
    throw new Error(`Timeline generation failed: ${(err as Error).message}`);
  }
}

async function synthesize(query: any, speakerId: number): Promise<ArrayBuffer> {
  const url = `${VOICEVOX_HOST}/synthesis?speaker=${speakerId}`;
  let response: Response;
  try {
    response = await fetchWithTimeout(
      url,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(query),
      },
      FETCH_TIMEOUT_MS,
    );
  } catch (err) {
    clearInitializedSpeakers();
    const error = err as Error;
    throw new Error(`VoiceVox unreachable at ${VOICEVOX_HOST}: ${error.message}`);
  }
  if (!response.ok) {
    const body = await response.text().catch(() => '');
    throw new Error(`synthesis failed (${response.status}): ${body}`);
  }
  return response.arrayBuffer();
}

await rpc.serve({
  methods: {
    speak: rpc.method({
      description: 'Make a character speak text with lip-synced audio via VoiceVox TTS',
      timeout: 300_000,
      input: z.object({
        personaId: z.string().min(1),
        text: z.union([z.string().min(1), z.array(z.string().min(1)).min(1)]),
      }),
      handler: async ({ personaId, text }) => {
        return withCharacterLock(personaId, async () => {
          const p = await Persona.load(personaId);
          const vrm = p.vrm();
          const assetId = await resolveAssetId(personaId);
          const settings = await loadSettings(assetId);

          await ensureSpeakerInitialized(settings.speakerId);

          const sentences = Array.isArray(text) ? text : [text];
          for (const sentence of sentences) {
            await speakSentence(vrm, sentence, settings);
          }

          return { success: true as const, sentences: sentences.length };
        });
      },
    }),
  },
});

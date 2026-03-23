import { z } from "zod";
import { Vrm, preferences, Webview, webviewSource, signals } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { normalizePhrase } from "@hmcs/sdk/wake-word-matcher";
import { KeyboardHookService } from "./lib/keyboard-hook.ts";
import { resolvePttKeycodes } from "./lib/key-mapping.ts";
import { SttHandler } from "./lib/stt-handler.ts";
import { PttAdapter } from "./lib/ptt-adapter.ts";
import { PermissionBridge } from "./lib/permission-bridge.ts";
import { SessionManager, type AgentSettings, type SessionState } from "./lib/session-manager.ts";
import { AlwaysOnAdapter } from "./lib/always-on-adapter.ts";
import type { InputAdapter } from "./lib/input-adapter.ts";

const DEFAULT_SETTINGS: AgentSettings = {
  wakeWords: [],
  shutdownWords: [],
  greetingPhrases: [],
  completionPhrases: [],
  errorPhrases: [],
  workingDirectories: { paths: [], default: 0 },
  listeningMode: "ptt",
  pttKey: null,
  approvalPhrases: ["はい", "yes", "ok", "allow"],
  denyPhrases: ["いいえ", "no", "deny", "cancel"],
  allowList: [],
  disallowedTools: [],
};

const keyboardHook = new KeyboardHookService();
const sttHandler = new SttHandler();
const permissionBridge = new PermissionBridge(sttHandler);

const sessionManagers = new Map<string, SessionManager>();
const pttAdapters = new Map<string, PttAdapter>();

async function loadApiKey(): Promise<string> {
  const apiKey = await preferences.load<string>("agent::api-key");
  if (!apiKey) throw new Error("API key not configured. Set 'agent::api-key' in preferences.");
  return apiKey;
}

async function loadCharacterSettings(characterId: string): Promise<AgentSettings> {
  const saved = await preferences.load<AgentSettings>("agent::" + characterId);
  return saved ? { ...DEFAULT_SETTINGS, ...saved } : { ...DEFAULT_SETTINGS };
}

async function registerCharacter(
  characterId: string,
  settings: AgentSettings,
  apiKey: string,
): Promise<void> {
  const sessionManager = new SessionManager(characterId, settings, permissionBridge, apiKey);
  sessionManagers.set(characterId, sessionManager);

  sttHandler.registerCharacter({
    wakeWordPhrases: settings.wakeWords.map(normalizePhrase),
    shutdownPhrases: settings.shutdownWords.map(normalizePhrase),
    approvalPhrases: settings.approvalPhrases.map(normalizePhrase),
    denyPhrases: settings.denyPhrases.map(normalizePhrase),
    characterId,
  });

  if (settings.wakeWords.length === 0) {
    console.warn(`[agent] No wake words configured for "${characterId}". Wake word detection will not work.`);
    emitAgentError(characterId, "No wake words configured. Open Agent Settings to add wake words.");
  }

  if (settings.listeningMode === "ptt" && settings.pttKey !== null) {
    const resolved = resolvePttKeycodes(settings.pttKey);
    if (resolved !== null) {
      const adapter = new PttAdapter(keyboardHook, sttHandler, resolved, characterId);
      pttAdapters.set(characterId, adapter);
    }
  }
}

async function registerAllCharacters(apiKey: string): Promise<void> {
  const snapshots = await Vrm.findAllDetailed();
  for (const snapshot of snapshots) {
    const settings = await loadCharacterSettings(snapshot.name);
    await registerCharacter(snapshot.name, settings, apiKey);
  }
}

async function openSessionUi(characterId: string): Promise<void> {
  const vrm = await Vrm.findByName(characterId);
  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    linkedVrm: vrm.entity,
  });
}

async function speakGreeting(characterId: string, phrases: string[]): Promise<void> {
  if (phrases.length === 0) return;
  const phrase = phrases[Math.floor(Math.random() * phrases.length)];
  await rpc
    .call({
      modName: "@hmcs/voicevox",
      method: "speak",
      body: { name: characterId, text: phrase },
    })
    .catch(() => console.warn("[agent] TTS unavailable for greeting"));
}

function resolveInputAdapter(
  characterId: string,
  settings: AgentSettings,
): InputAdapter | null {
  if (settings.listeningMode === "ptt") {
    return pttAdapters.get(characterId) ?? null;
  }
  return new AlwaysOnAdapter(sttHandler, characterId);
}

function emitAgentError(characterId: string, message: string): void {
  console.error(`[agent] ${characterId}: ${message}`);
  signals.send("agent:error", { characterId, message });
}

async function startSession(characterId: string): Promise<void> {
  const manager = sessionManagers.get(characterId);
  if (!manager) return;

  const adapter = resolveInputAdapter(characterId, manager.settings);
  if (!adapter) {
    emitAgentError(
      characterId,
      "PTT key not configured. Open Agent Settings to set a push-to-talk key.",
    );
    return;
  }

  const vrm = await Vrm.findByName(characterId);
  const sdkPersona = await vrm.persona();
  const persona = { name: characterId, ...sdkPersona };

  await openSessionUi(characterId);
  await speakGreeting(characterId, manager.settings.greetingPhrases);
  await manager.start(persona, adapter);
}

function setupWakeWordHandler(): void {
  sttHandler.onWakeWord = (characterId) => {
    startSession(characterId).catch((err) =>
      console.error(`[agent] Failed to start session for ${characterId}:`, err),
    );
  };
}

function setupShutdownWordHandler(): void {
  sttHandler.onShutdownWord = (characterId) => {
    const manager = sessionManagers.get(characterId);
    manager
      ?.interrupt()
      .catch((err) =>
        console.error(`[agent] Failed to interrupt session for ${characterId}:`, err),
      );
  };
}

function buildRpcMethods() {
  return {
    "approve-permission": rpc.method({
      description: "Approve or deny a pending permission request",
      input: z.object({ requestId: z.string(), approved: z.boolean() }),
      handler: async ({ requestId, approved }) => {
        permissionBridge.resolvePermission(requestId, approved);
        return { success: true as const };
      },
    }),
    "answer-question": rpc.method({
      description: "Answer a pending question request",
      input: z.object({ requestId: z.string(), answers: z.record(z.string()) }),
      handler: async ({ requestId, answers }) => {
        permissionBridge.resolveQuestion(requestId, answers);
        return { success: true as const };
      },
    }),
    status: rpc.method({
      description: "Get the current session state for all characters",
      handler: async () => {
        const result: Record<string, SessionState> = {};
        for (const [id, manager] of sessionManagers) {
          result[id] = manager.getState();
        }
        return result;
      },
    }),
    "start-session": rpc.method({
      description: "Manually start an agent session for a character",
      input: z.object({ characterId: z.string() }),
      handler: async ({ characterId }) => {
        await startSession(characterId);
        return { success: true as const };
      },
    }),
    "stop-session": rpc.method({
      description: "Stop an active agent session for a character",
      input: z.object({ characterId: z.string() }),
      handler: async ({ characterId }) => {
        const manager = sessionManagers.get(characterId);
        await manager?.stop();
        return { success: true as const };
      },
    }),
  };
}

async function startKeyboardHook(): Promise<void> {
  const started = keyboardHook.start();
  if (!started) {
    console.warn("[agent] Keyboard hook failed to start — falling back to wake-word-only mode");
  }
}

async function startStt(): Promise<void> {
  try {
    await sttHandler.start();
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    console.error("[agent] STT session start failed:", message);
    signals.send("agent:error", {
      characterId: "*",
      message: `STT startup failed: ${message}`,
    });
  }
}

async function shutdown(): Promise<void> {
  console.log("[agent] Shutting down...");

  for (const manager of sessionManagers.values()) {
    await manager.stop().catch(() => {});
  }

  for (const adapter of pttAdapters.values()) {
    adapter.forceFlush();
  }

  keyboardHook.stop();
  sttHandler.close();
}

async function main(): Promise<void> {
  let apiKey: string;
  try {
    apiKey = await loadApiKey();
  } catch {
    console.error("[agent] API key not configured. Agent service will not start.");
    signals.send("agent:error", {
      characterId: "*",
      message: "API key not configured. Open Agent Settings to set your Anthropic API key.",
    });
    return;
  }

  await startKeyboardHook();
  await startStt();
  await registerAllCharacters(apiKey);

  setupWakeWordHandler();
  setupShutdownWordHandler();

  await rpc.serve({ methods: buildRpcMethods() });
}

main().catch((err) => console.error("[agent] Fatal startup error:", err));

process.once("SIGTERM", () => {
  shutdown().catch((err) => console.error("[agent] Shutdown error:", err));
});

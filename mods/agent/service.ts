import { z } from "zod";
import { Vrm, preferences, Webview, webviewSource, signals } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { KeyboardHookService } from "./lib/keyboard-hook.ts";
import { resolvePttKeycodes } from "./lib/key-mapping.ts";
import { PttAdapter } from "./lib/ptt-adapter.ts";
import { PermissionBridge } from "./lib/permission-bridge.ts";
import { SessionManager } from "./lib/session-manager.ts";
import {
  type AgentSettings,
  type SessionState,
  DEFAULT_SETTINGS,
} from "./lib/types.ts";

const keyboardHook = new KeyboardHookService();
const permissionBridge = new PermissionBridge();

const sessionManagers = new Map<string, SessionManager>();

async function loadApiKey(): Promise<string> {
  const apiKey = await preferences.load<string>("agent::api-key");
  if (!apiKey)
    throw new Error(
      "API key not configured. Set 'agent::api-key' in preferences.",
    );
  return apiKey;
}

async function loadCharacterSettings(
  characterId: string,
): Promise<AgentSettings> {
  const saved = await preferences.load<AgentSettings>("agent::" + characterId);
  return saved ? { ...DEFAULT_SETTINGS, ...saved } : { ...DEFAULT_SETTINGS };
}

async function registerCharacter(
  characterId: string,
  apiKey: string,
): Promise<void> {
  const settings = await loadCharacterSettings(characterId);
  const sessionManager = new SessionManager(
    characterId,
    settings,
    permissionBridge,
    apiKey,
  );
  sessionManagers.set(characterId, sessionManager);
}

async function registerAllCharacters(apiKey: string): Promise<void> {
  const snapshots = await Vrm.findAllDetailed();
  for (const snapshot of snapshots) {
    await registerCharacter(snapshot.name, apiKey);
  }
}

async function openSessionUi(characterId: string): Promise<void> {
  const vrm = await Vrm.findByName(characterId);
  await Webview.open({
    source: webviewSource.local("agent:session-ui"),
    size: [0.6, 0.8],
    viewportSize: [400, 500],
    linkedVrm: vrm.entity,
  });
}

async function speakGreeting(
  characterId: string,
  phrases: string[],
): Promise<void> {
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

function emitAgentError(characterId: string, message: string): void {
  console.error(`[agent] ${characterId}: ${message}`);
  signals.send("agent:error", { characterId, message });
}

async function startSession(characterId: string): Promise<void> {
  const manager = sessionManagers.get(characterId);
  if (!manager) return;

  const settings = manager.settings;
  const resolvedKey = resolveSessionPttKey(characterId, settings);
  if (!resolvedKey) return;

  const vrm = await Vrm.findByName(characterId);
  const sdkPersona = await vrm.persona();
  const persona = { name: characterId, ...sdkPersona };

  await openSessionUi(characterId);
  await speakGreeting(characterId, settings.greetingPhrases);

  const pttAdapter = new PttAdapter(keyboardHook, resolvedKey, characterId);
  wireVoiceApproval(pttAdapter, permissionBridge, settings);
  await manager.start(persona, pttAdapter);
}

function resolveSessionPttKey(
  characterId: string,
  settings: AgentSettings,
): ReturnType<typeof resolvePttKeycodes> {
  if (!settings.pttKey) {
    emitAgentError(
      characterId,
      "PTT key not configured. Open Agent Settings to set a push-to-talk key.",
    );
    return null;
  }
  const resolved = resolvePttKeycodes(settings.pttKey);
  if (!resolved) {
    emitAgentError(characterId, "PTT key could not be resolved.");
    return null;
  }
  return resolved;
}

function wireVoiceApproval(
  pttAdapter: PttAdapter,
  bridge: PermissionBridge,
  settings: AgentSettings,
): void {
  bridge.onPermissionWaitStart = (requestId: string) => {
    pttAdapter.setMode("permission_wait", (text) => {
      const approved = matchApprovalPhrase(text, settings);
      bridge.resolveExternally(requestId, approved);
    });
  };
  bridge.onPermissionResolved = () => {
    pttAdapter.setMode("normal");
  };
}

function matchApprovalPhrase(text: string, settings: AgentSettings): boolean {
  const lower = text.toLowerCase();
  return settings.approvalPhrases.some((phrase) =>
    lower.includes(phrase.toLowerCase()),
  );
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
      input: z.object({
        requestId: z.string(),
        answers: z.record(z.string()),
      }),
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
    console.warn("[agent] Keyboard hook failed to start.");
  }
}

async function shutdown(): Promise<void> {
  console.log("[agent] Shutting down...");
  for (const manager of sessionManagers.values()) {
    await manager.stop().catch(() => {});
  }
  keyboardHook.stop();
}

async function main(): Promise<void> {
  let apiKey: string;
  try {
    apiKey = await loadApiKey();
  } catch {
    console.error(
      "[agent] API key not configured. Agent service will not start.",
    );
    signals.send("agent:error", {
      characterId: "*",
      message:
        "API key not configured. Open Agent Settings to set your Anthropic API key.",
    });
    return;
  }

  await startKeyboardHook();
  await registerAllCharacters(apiKey);
  await rpc.serve({ methods: buildRpcMethods() });
}

main().catch((err) => console.error("[agent] Fatal startup error:", err));

process.once("SIGTERM", () => {
  shutdown().catch((err) => console.error("[agent] Shutdown error:", err));
});

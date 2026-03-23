import { query } from "@anthropic-ai/claude-agent-sdk";
import { preferences, signals } from "@hmcs/sdk";
import { rpc } from "@hmcs/sdk/rpc";
import { mkdirSync } from "node:fs";
import { homedir } from "node:os";
import path from "node:path";
import type { PermissionBridge } from "./permission-bridge.ts";
import type { PttAdapter } from "./ptt-adapter.ts";

export type SessionState = "idle" | "running" | "interrupted" | "recovering";

export interface AgentSettings {
  wakeWords: string[];
  shutdownWords: string[];
  greetingPhrases: string[];
  completionPhrases: string[];
  errorPhrases: string[];
  workingDirectories: { paths: string[]; default: number };
  listeningMode: "ptt" | "always-on";
  pttKey: string | null;
  approvalPhrases: string[];
  denyPhrases: string[];
  allowList: string[];
  disallowedTools: string[];
}

/** Persona type from @hmcs/sdk vrm.ts */
interface Persona {
  name: string;
  profile: string;
  personality?: string | null;
}

/** Opaque handle to a running `query()` call from the Claude Agent SDK. */
type QueryHandle = AsyncIterable<any> & {
  interrupt?: () => Promise<void>;
  close?: () => void;
};

const SHUTDOWN_DEADLINE_MS = 5_000;
const SESSION_PREF_PREFIX = "agent::session::";

export class SessionManager {
  readonly characterId: string;
  readonly settings: AgentSettings;

  private state: SessionState = "idle";
  private sessionId: string | null = null;
  private currentQuery: QueryHandle | null = null;
  private pttAdapter: PttAdapter | null = null;
  private readonly permissionBridge: PermissionBridge;
  private readonly apiKey: string;

  constructor(
    characterId: string,
    settings: AgentSettings,
    permissionBridge: PermissionBridge,
    apiKey: string,
  ) {
    this.characterId = characterId;
    this.settings = settings;
    this.permissionBridge = permissionBridge;
    this.apiKey = apiKey;
  }

  getState(): SessionState {
    return this.state;
  }

  async start(persona: Persona, pttAdapter: PttAdapter): Promise<void> {
    if (this.state === "running") return;
    this.pttAdapter = pttAdapter;

    const workDir = this.resolveWorkingDirectory();
    mkdirSync(workDir, { recursive: true });

    const savedSessionId = await preferences.load<string>(
      `${SESSION_PREF_PREFIX}${this.characterId}`,
    );

    this.currentQuery = query({
      prompt: pttAdapter.createAsyncGenerator(),
      options: buildQueryOptions({
        characterId: this.characterId,
        persona,
        workDir,
        savedSessionId,
        disallowedTools: this.settings.disallowedTools,
        permissionHandler: this.permissionBridge.createHandler(this.characterId),
        apiKey: this.apiKey,
      }),
    });

    this.state = "running";
    this.emitStatus("idle");
    this.processMessages().catch((err) => this.handleSessionError(err));
  }

  async interrupt(): Promise<void> {
    if (this.state !== "running" || !this.currentQuery) return;
    this.state = "interrupted";
    this.permissionBridge.cancelAll();
    await this.currentQuery.interrupt?.();
    this.state = "running";
  }

  async stop(): Promise<void> {
    if (!this.currentQuery) {
      this.state = "idle";
      return;
    }
    this.permissionBridge.cancelAll();
    await this.drainWithDeadline();
    this.teardown();
  }

  private async processMessages(): Promise<void> {
    if (!this.currentQuery) return;
    for await (const msg of this.currentQuery) {
      this.handleMessage(msg);
    }
    this.handleProcessExit(0);
  }

  private handleMessage(msg: any): void {
    switch (msg.type) {
      case "system":
        this.handleSystemMessage(msg);
        break;
      case "assistant":
        this.handleAssistantMessage(msg);
        break;
      case "tool_use_summary":
        this.handleToolUseSummary(msg);
        break;
      case "result":
        this.handleResultMessage(msg);
        break;
    }
  }

  private handleSystemMessage(msg: any): void {
    if (msg.session_id) {
      this.sessionId = msg.session_id;
      preferences.save(
        `${SESSION_PREF_PREFIX}${this.characterId}`,
        this.sessionId,
      );
    }
  }

  private handleAssistantMessage(msg: any): void {
    this.emitStatus("thinking");
    const text = extractTextContent(msg);
    if (text) {
      this.emitLog("assistant", text);
    }
  }

  private handleToolUseSummary(msg: any): void {
    this.emitStatus("executing");
    const preview = JSON.stringify(msg.tool_input).slice(0, 100);
    this.emitLog("tool", `${msg.tool_name}: ${preview}`);
  }

  private handleResultMessage(msg: any): void {
    this.emitStatus("idle");
    if (this.sessionId) {
      preferences.save(
        `${SESSION_PREF_PREFIX}${this.characterId}`,
        this.sessionId,
      );
    }
    const phrases =
      msg.subtype === "success"
        ? this.settings.completionPhrases
        : this.settings.errorPhrases;
    this.speakRandomPhrase(phrases);
  }

  private handleSessionError(err: unknown): void {
    console.error(`[agent] Session error for ${this.characterId}:`, err);
    this.handleProcessExit(1);
  }

  private handleProcessExit(code: number): void {
    if (code !== 0) {
      this.emitLog("error", `Agent process exited with code ${code}`);
      this.speakRandomPhrase(this.settings.errorPhrases);
    }
    this.state = "idle";
    this.currentQuery = null;
  }

  private async drainWithDeadline(): Promise<void> {
    try {
      await this.currentQuery!.interrupt?.();
      await new Promise((r) => setTimeout(r, SHUTDOWN_DEADLINE_MS));
    } catch {}
  }

  private teardown(): void {
    this.currentQuery?.close?.();
    this.currentQuery = null;
    this.pttAdapter?.close();
    this.pttAdapter = null;
    this.state = "idle";
  }

  private resolveWorkingDirectory(): string {
    const dirs = this.settings.workingDirectories;
    return (
      dirs.paths[dirs.default] ??
      path.join(homedir(), ".homunculus", "agents", this.characterId)
    );
  }

  private emitStatus(state: string): void {
    signals.send("agent:status", { characterId: this.characterId, state });
  }

  private emitLog(type: string, message: string): void {
    signals.send("agent:log", {
      characterId: this.characterId,
      type,
      message,
      timestamp: Date.now(),
    });
  }

  private speakRandomPhrase(phrases: string[]): void {
    if (phrases.length === 0) return;
    const phrase = phrases[Math.floor(Math.random() * phrases.length)];
    rpc
      .call({
        modName: "@hmcs/voicevox",
        method: "speak",
        body: { name: this.characterId, text: phrase },
      })
      .catch(() => this.emitLog("warning", "TTS unavailable"));
  }
}

interface QueryContext {
  characterId: string;
  persona: Persona;
  workDir: string;
  savedSessionId: string | null | undefined;
  disallowedTools: string[];
  permissionHandler: ReturnType<PermissionBridge["createHandler"]>;
  apiKey: string;
}

function buildQueryOptions(ctx: QueryContext): Record<string, unknown> {
  const options: Record<string, unknown> = {
    systemPrompt: buildCharacterPrompt(ctx.persona),
    cwd: ctx.workDir,
    mcpServers: {
      homunculus: { type: "http", url: "http://localhost:3100/mcp" },
    },
    hooks: {
      PreToolUse: [buildReadOnlyHook()],
    },
    disallowedTools: ctx.disallowedTools,
    canUseTool: ctx.permissionHandler,
    env: { ...process.env, NODE_OPTIONS: "", ANTHROPIC_API_KEY: ctx.apiKey },
    maxTurns: 100,
  };
  if (ctx.savedSessionId) {
    options.resume = ctx.savedSessionId;
  }
  return options;
}

function buildReadOnlyHook(): Record<string, unknown> {
  return {
    matcher: "^(Read|Glob|Grep|mcp__homunculus__list|mcp__homunculus__get)",
    hooks: [() => ({ hookSpecificOutput: { permissionDecision: "allow" } })],
  };
}

function buildCharacterPrompt(persona: Persona): string {
  return [
    `あなたは「${persona.name}」です。`,
    persona.profile && `プロフィール: ${persona.profile}`,
    persona.personality && `性格: ${persona.personality}`,
    `Desktop Homunculusのキャラクターとして、ユーザーの指示に従って作業を行ってください。`,
    `応答は簡潔にしてください。`,
  ]
    .filter(Boolean)
    .join("\n");
}

function extractTextContent(msg: any): string {
  const textBlocks = msg.message?.content?.filter?.(
    (b: any) => b.type === "text",
  );
  if (!textBlocks?.length) return "";
  return textBlocks.map((b: any) => b.text).join("");
}

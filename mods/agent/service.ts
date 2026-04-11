import { mkdirSync } from 'node:fs';
import { homedir } from 'node:os';
import path from 'node:path';
import { preferences, Persona as SdkPersona, signals } from '@hmcs/sdk';
import { rpc } from '@hmcs/sdk/rpc';
import { z } from 'zod';
import { buildFrontmanPrompt, createFrontmanRuntime } from './lib/frontman.ts';
import { currentBranch, gitExec, isGitRepo, listBranches } from './lib/git.ts';
import { type ResolvedPttKey, resolvePttKeycodes } from './lib/key-mapping.ts';
import { KeyboardHookService } from './lib/keyboard-hook.ts';
import type { AgentRuntime } from './lib/runtime/agent-runtime.ts';
import { CodexAppServerProcess } from './lib/runtime/codex-appserver-process.ts';
import { SessionManager } from './lib/session-manager.ts';
import { type PersistLogEntry, SessionPersistence } from './lib/session-persistence.ts';
import { type AgentSettings, DEFAULT_SETTINGS, type Persona } from './lib/types.ts';
import type { WorktreeContext } from './lib/prompt.ts';
import { buildWorkerPrompt, createWorkerRuntime } from './lib/worker.ts';
import { WORKTREE_NAME_PATTERN, WorktreeManager } from './lib/worktree-manager.ts';

const keyboardHook = new KeyboardHookService();
const persistence = new SessionPersistence();
const sessionManager = new SessionManager(persistence, keyboardHook);

let appServerProcess: CodexAppServerProcess | null = null;

function getAppServerProcess(): CodexAppServerProcess {
  if (!appServerProcess) {
    appServerProcess = new CodexAppServerProcess();
  }
  return appServerProcess;
}

let currentApiKey: string | null = null;

async function loadApiKey(): Promise<string> {
  const apiKey = await preferences.load<string>('agent::api-key');
  if (!apiKey) throw new Error("API key not configured. Set 'agent::api-key' in preferences.");
  return apiKey;
}

async function loadPersonaSettings(personaId: string): Promise<AgentSettings> {
  const saved = await preferences.load<Record<string, unknown>>(`agent::${personaId}`);
  if (!saved) return { ...DEFAULT_SETTINGS };

  // Migrate workingDirectories → workspaces
  if ('workingDirectories' in saved && !('workspaces' in saved)) {
    const wd = saved.workingDirectories as { paths: string[]; default: number };
    saved.workspaces = {
      paths: wd.paths,
      selection: { workspaceIndex: wd.default, worktreeName: null },
    };
    delete saved.workingDirectories;
    await preferences.save(`agent::${personaId}`, { ...DEFAULT_SETTINGS, ...saved });
  }

  return { ...DEFAULT_SETTINGS, ...(saved as Partial<AgentSettings>) };
}

async function startKeyboardHook(): Promise<void> {
  const started = keyboardHook.start();
  if (!started) {
    console.warn('[agent] Keyboard hook failed to start.');
  }
}

async function scanOrphanedWorktrees(): Promise<void> {
  const snapshots = await SdkPersona.list();
  for (const snapshot of snapshots) {
    const personaId = snapshot.id;
    if (sessionManager.hasFrontman(personaId)) continue;
    const settings = await loadPersonaSettings(personaId);
    for (const wsPath of settings.workspaces.paths) {
      try {
        if (!(await isGitRepo(wsPath))) continue;
        const manager = new WorktreeManager(wsPath);
        const worktrees = await manager.list();
        for (const wt of worktrees) {
          const hasChanges = await manager.hasUncommittedChanges(wt.name);
          if (hasChanges) {
            await signals.send('agent:worktree', {
              personaId,
              state: 'orphaned',
              worktreeName: wt.name,
              workspacePath: wsPath,
            });
            sessionManager.emitLog(
              personaId,
              'warning',
              `Orphaned worktree detected: ${wt.name} in ${wsPath} (has uncommitted changes)`,
            );
          }
        }
      } catch {
        // Skip workspaces that can't be scanned
      }
    }
  }
}

async function startSession(
  personaId: string,
  contextSessionUuid?: string,
): Promise<PersistLogEntry[]> {
  const settings = await loadPersonaSettings(personaId);
  assertCanStartSession(personaId, settings);

  const resolvedKey = resolvePttKeyOptional(settings);
  const persona = await loadPersona(personaId);
  const workDir = resolveWorkingDirectory(personaId, settings);
  mkdirSync(workDir, { recursive: true });

  const selection = settings.workspaces.selection;
  if (selection.worktreeName) {
    await signals.send('agent:worktree', {
      personaId,
      state: 'created',
      worktreeName: selection.worktreeName,
      workspacePath: settings.workspaces.paths[selection.workspaceIndex],
    });
  }

  const basePath = settings.workspaces.paths[selection.workspaceIndex];
  const branchName = basePath ? await resolveCurrentBranch(basePath, selection.worktreeName) : null;

  // Read previous session entries for UI replay. Frontman does not use
  // session context injection — that will be handled by Workers (Phase 5).
  let replayEntries: PersistLogEntry[] = [];
  if (basePath && branchName) {
    const contextUuid =
      contextSessionUuid ??
      (await persistence.findLatestSessionUuid(basePath, personaId, branchName));
    if (contextUuid) {
      const entries = await persistence.read(basePath, personaId, branchName, contextUuid);
      if (entries.length > 0) {
        replayEntries = entries;
      }
    }
  }

  const prompt = buildFrontmanPrompt(persona);
  const runtime = createFrontmanRuntime({
    settings,
    prompt,
    apiKey: currentApiKey,
    workDir,
    appServerProcess: getAppServerProcess(),
  });

  sessionManager.startFrontman(personaId, runtime);
  sessionManager.attachTextQueue(personaId);

  if (basePath && branchName) {
    const handle = await persistence.create({
      workspacePath: basePath,
      personaId,
      branchName,
    });
    sessionManager.attachSessionHandle(personaId, handle);
    persistence.cleanup(basePath, personaId).catch(() => {});
  }

  launchSessionLoop(personaId, runtime, settings, resolvedKey);
  return replayEntries;
}

function assertCanStartSession(personaId: string, settings: AgentSettings): void {
  if (settings.runtime === 'sdk' && !currentApiKey) {
    throw new Error('API key not configured. Open Agent Settings to set your Anthropic API key.');
  }
  if (sessionManager.hasFrontman(personaId)) {
    throw new Error(`Session already active for "${personaId}".`);
  }
}

function launchSessionLoop(
  personaId: string,
  runtime: AgentRuntime,
  settings: AgentSettings,
  resolvedKey: ResolvedPttKey | null,
): void {
  sessionManager
    .runFrontmanLoop(personaId, runtime, settings, resolvedKey)
    .catch((err) => handleSessionCrash(personaId, err));
}

function handleSessionCrash(personaId: string, err: unknown): void {
  if (!isAbortError(err)) {
    console.error(`[agent] Session error for ${personaId}:`, err);
    sessionManager.emitLog(personaId, 'error', extractErrorMessage(err));
  }
  // runFrontmanLoop's finally block already tore down persistence, queues, and
  // tracking state. Just emit a terminal status if this wasn't a clean abort.
  if (!isAbortError(err)) {
    sessionManager.emitStatus(personaId, 'idle', 'crashed');
  }
}

function resolvePttKeyOptional(settings: AgentSettings): ResolvedPttKey | null {
  if (!settings.pttKey) return null;
  return resolvePttKeycodes(settings.pttKey) ?? null;
}

async function loadPersona(personaId: string): Promise<Persona> {
  const p = await SdkPersona.load(personaId);
  const snapshot = await p.snapshot();
  return {
    name: snapshot.name || personaId,
    age: snapshot.age ?? null,
    gender: snapshot.gender ?? 'unknown',
    firstPersonPronoun: snapshot.firstPersonPronoun ?? null,
    profile: snapshot.profile ?? '',
    personality: snapshot.personality ?? null,
  };
}

function resolveWorkingDirectory(personaId: string, settings: AgentSettings): string {
  const { paths, selection } = settings.workspaces;
  const basePath = paths[selection.workspaceIndex];
  if (!basePath) return path.join(homedir(), '.homunculus', 'agents', personaId);
  if (selection.worktreeName) {
    return path.join(basePath, '.hmcs/worktrees', selection.worktreeName);
  }
  return basePath;
}

/** Resolve the current git branch for session scoping. */
async function resolveCurrentBranch(
  workspacePath: string,
  worktreeName: string | null,
): Promise<string | null> {
  try {
    if (worktreeName) {
      const manager = new WorktreeManager(workspacePath);
      const worktrees = await manager.list();
      const wt = worktrees.find((w) => w.name === worktreeName);
      return wt?.branch ?? null;
    }
    return await currentBranch(workspacePath);
  } catch {
    return null;
  }
}

async function stopSession(personaId: string): Promise<void> {
  if (!sessionManager.hasFrontman(personaId)) return;
  await sessionManager.stopPersonaSessions(personaId);
  sessionManager.emitStatus(personaId, 'idle', 'stopped');

  if (appServerProcess && appServerProcess.refCount === 0) {
    appServerProcess.shutdown();
    appServerProcess = null;
  }
}

function isAbortError(e: unknown): boolean {
  return e instanceof DOMException && e.name === 'AbortError';
}

function extractErrorMessage(err: unknown): string {
  if (!(err instanceof Error)) return String(err);
  const cause = err.cause instanceof Error ? `: ${err.cause.message}` : '';
  return `${err.message}${cause}`;
}

function resolveWorkerWorkDir(
  personaId: string,
  settings: AgentSettings,
  worktreeName: string | null,
): string {
  if (worktreeName) {
    const basePath = settings.workspaces.paths[settings.workspaces.selection.workspaceIndex];
    if (!basePath) return path.join(homedir(), '.homunculus', 'agents', personaId);
    return path.join(basePath, '.hmcs/worktrees', worktreeName);
  }
  return resolveWorkingDirectory(personaId, settings);
}

async function readWorktreeBaseBranch(worktreePath: string): Promise<string> {
  try {
    const result = await gitExec(worktreePath, ['config', 'hmcs.baseBranch']);
    return result.trim() || 'main';
  } catch {
    return 'main';
  }
}

async function buildWorktreeContext(
  worktreeName: string,
  workDir: string,
): Promise<WorktreeContext> {
  const baseBranch = await readWorktreeBaseBranch(workDir);
  return { worktreeName, baseBranch, worktreePath: workDir };
}

function buildRpcMethods() {
  return {
    'approve-permission': rpc.method({
      description: 'Approve or deny a pending permission request',
      input: z.object({
        requestId: z.string(),
        approved: z.boolean(),
        decision: z.union([z.string(), z.record(z.unknown())]).optional(),
      }),
      handler: async ({ requestId, approved, decision }) => {
        const ok = sessionManager.resolvePermission(requestId, { approved, decision });
        if (!ok) {
          throw new Error('No pending approval for this request');
        }
        return {};
      },
    }),
    'answer-question': rpc.method({
      description: 'Answer a pending question request',
      input: z.object({
        requestId: z.string(),
        answers: z.record(z.string()),
      }),
      handler: async ({ requestId, answers }) => {
        sessionManager.resolveQuestion(requestId, answers);
        return {};
      },
    }),
    'send-message': rpc.method({
      description: 'Send a text message to the agent',
      input: z.object({
        personaId: z.string(),
        text: z.string().min(1),
        contextSessionUuid: z.string().optional(),
      }),
      handler: async ({ personaId, text, contextSessionUuid }) => {
        let replayEntries: PersistLogEntry[] = [];
        if (!sessionManager.hasFrontman(personaId)) {
          replayEntries = await startSession(personaId, contextSessionUuid);
        }
        sessionManager.resolveInterrupt(personaId);
        sessionManager.sendMessage(personaId, text);
        return { replayEntries };
      },
    }),
    'set-text-focus': rpc.method({
      description: 'Report whether an editable element currently has focus in the WebView',
      input: z.object({
        personaId: z.string(),
        focused: z.boolean(),
      }),
      handler: async ({ personaId, focused }) => {
        sessionManager.setTextFocus(personaId, focused);
        return {};
      },
    }),
    status: rpc.method({
      description: 'Get the current session state for all personas',
      handler: async () => {
        const result: Record<string, string> = {};
        for (const id of sessionManager.listActivePersonas()) {
          result[id] = 'active';
        }
        return result;
      },
    }),
    'get-session-status': rpc.method({
      description: 'Get the session status for a specific persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        const status = sessionManager.hasFrontman(personaId) ? 'active' : 'idle';
        return { status } as const;
      },
    }),
    'start-session': rpc.method({
      description: 'Manually start an agent session for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        const replayEntries = await startSession(personaId);
        return { replayEntries };
      },
    }),
    'stop-session': rpc.method({
      description: 'Stop an active agent session for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        await stopSession(personaId);
        return {};
      },
    }),
    'interrupt-session': rpc.method({
      description: 'Interrupt the current agent execution for a persona',
      input: z.object({ personaId: z.string() }),
      handler: async ({ personaId }) => {
        sessionManager.resolveInterrupt(personaId);
        return {};
      },
    }),
    'list-worktrees': rpc.method({
      description: 'List worktrees for a workspace',
      input: z.object({ workspacePath: z.string() }),
      handler: async ({ workspacePath }) => {
        const manager = new WorktreeManager(workspacePath);
        const worktrees = await manager.list();
        const results = await Promise.all(
          worktrees.map(async (wt) => {
            const status = await manager.status(wt.name);
            return { ...wt, ...status };
          }),
        );
        return { worktrees: results };
      },
    }),
    'add-worktree': rpc.method({
      description: 'Create a new worktree in a workspace',
      input: z.object({
        workspacePath: z.string(),
        name: z
          .string()
          .min(1)
          .regex(
            WORKTREE_NAME_PATTERN,
            'Name must contain only alphanumeric characters, hyphens, underscores, and dots',
          ),
        branch: z.string().min(1),
      }),
      handler: async ({ workspacePath, name, branch }) => {
        const manager = new WorktreeManager(workspacePath);
        const info = await manager.create(name, branch);
        return { worktree: info };
      },
    }),
    'remove-worktree': rpc.method({
      description: 'Remove a worktree (optionally merge first)',
      input: z.object({
        workspacePath: z.string(),
        name: z.string(),
        action: z.enum(['remove', 'merge']),
      }),
      handler: async ({ workspacePath, name, action }) => {
        const manager = new WorktreeManager(workspacePath);
        if (action === 'merge') {
          const result = await manager.merge(name);
          if (!result.success) {
            throw new Error(result.error);
          }
          return {};
        }
        await manager.remove(name);
        return {};
      },
    }),
    'list-branches': rpc.method({
      description: 'List git branches for a workspace',
      input: z.object({ workspacePath: z.string() }),
      handler: async ({ workspacePath }) => {
        const branches = await listBranches(workspacePath);
        const current = await currentBranch(workspacePath);
        return { branches, current };
      },
    }),
    'worktree-status': rpc.method({
      description: 'Get status of a specific worktree',
      input: z.object({ workspacePath: z.string(), name: z.string() }),
      handler: async ({ workspacePath, name }) => {
        const manager = new WorktreeManager(workspacePath);
        return await manager.status(name);
      },
    }),
    'get-current-branch': rpc.method({
      description: 'Resolve the current git branch for a workspace',
      input: z.object({
        workspacePath: z.string(),
        worktreeName: z.string().nullable(),
      }),
      handler: async ({ workspacePath, worktreeName }) => {
        const branchName = await resolveCurrentBranch(workspacePath, worktreeName);
        if (!branchName) {
          throw new Error('Not a git repository or branch could not be resolved');
        }
        return { branchName };
      },
    }),
    'list-sessions': rpc.method({
      description: 'List past sessions for a persona on a branch',
      input: z.object({
        workspacePath: z.string(),
        personaId: z.string(),
        branchName: z.string(),
      }),
      handler: async ({ workspacePath, personaId, branchName }) => {
        const sessions = await persistence.list(workspacePath, personaId, branchName);
        return { sessions };
      },
    }),
    'get-session-logs': rpc.method({
      description: 'Read the full log entries for a past session',
      input: z.object({
        workspacePath: z.string(),
        personaId: z.string(),
        branchName: z.string(),
        uuid: z.string(),
      }),
      handler: async ({ workspacePath, personaId, branchName, uuid }) => {
        const entries = await persistence.read(workspacePath, personaId, branchName, uuid);
        return { entries };
      },
    }),
    'delegate-task': rpc.method({
      description:
        'Spawn a Worker to execute an implementation task. Returns a taskId for tracking.',
      input: z.object({
        personaId: z.string(),
        description: z.string().min(1),
        worktreeName: z.string().nullable(),
      }),
      handler: async ({ personaId, description, worktreeName }) => {
        const settings = await loadPersonaSettings(personaId);
        const persona = await loadPersona(personaId);

        const workDir = resolveWorkerWorkDir(personaId, settings, worktreeName);

        const worktreeCtx = worktreeName
          ? await buildWorktreeContext(worktreeName, workDir)
          : undefined;

        const prompt = buildWorkerPrompt(persona, {
          taskDescription: description,
          worktree: worktreeCtx,
        });

        const { taskId } = await sessionManager.delegateTask({
          personaId,
          description,
          worktreeName,
          createRuntime: () =>
            createWorkerRuntime({
              settings,
              prompt,
              apiKey: currentApiKey,
              workDir,
              appServerProcess: getAppServerProcess(),
            }),
        });
        return { taskId };
      },
    }),
    'cancel-task': rpc.method({
      description: 'Cancel a running Worker task.',
      input: z.object({
        personaId: z.string(),
        taskId: z.string(),
      }),
      handler: async ({ personaId, taskId }) => {
        sessionManager.cancelTask(personaId, taskId);
        return {};
      },
    }),
    'get-task-status': rpc.method({
      description: 'Get the status of a Worker task.',
      input: z.object({
        personaId: z.string(),
        taskId: z.string(),
      }),
      handler: async ({ personaId, taskId }) => {
        const task = sessionManager.getTaskStatus(personaId, taskId);
        if (!task) return { found: false };
        return {
          found: true,
          status: task.status,
          description: task.description,
          worktreeName: task.worktreeName,
          startedAt: task.startedAt,
          endedAt: task.endedAt,
          errorMessage: task.errorMessage,
        };
      },
    }),
  };
}

async function shutdown(): Promise<void> {
  console.log('[agent] Shutting down...');
  sessionManager.stopAllSessions();
  keyboardHook.stop();

  if (appServerProcess) {
    appServerProcess.shutdown();
    appServerProcess = null;
  }
}

async function main(): Promise<void> {
  try {
    currentApiKey = await loadApiKey();
  } catch {
    console.warn('[agent] API key not configured. Sessions require an API key.');
  }

  await startKeyboardHook();
  await scanOrphanedWorktrees();
  await rpc.serve({ methods: buildRpcMethods() });
}

main().catch((err) => console.error('[agent] Fatal startup error:', err));

process.once('SIGTERM', () => {
  shutdown().catch((err) => console.error('[agent] Shutdown error:', err));
});

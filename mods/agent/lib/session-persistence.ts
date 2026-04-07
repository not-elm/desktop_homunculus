import { mkdir, writeFile, appendFile } from "node:fs/promises";
import { join } from "node:path";
import { randomUUID } from "node:crypto";

/** Opaque handle to an active session file. */
export interface SessionHandle {
  uuid: string;
  filePath: string;
}

/** Metadata for a session, derived from its JSONL header + preview scan. */
export interface SessionMeta {
  uuid: string;
  startedAt: number;
  preview: string | null;
}

/** A single log entry persisted to JSONL. Wider than LogEntry to allow any type string. */
export interface PersistLogEntry {
  type: string;
  message: string;
  timestamp: number;
  source?: string;
}

const SESSION_TTL_DAYS = 90;

export class SessionPersistence {
  /** Resolve the directory for a given persona + branch scope. */
  private sessionDir(workspacePath: string, personaId: string, branchName: string): string {
    return join(workspacePath, ".hmcs", "sessions", personaId, branchName);
  }

  /** Create a new session file and write the JSONL header. */
  async create(params: {
    workspacePath: string;
    personaId: string;
    branchName: string;
  }): Promise<SessionHandle> {
    const dir = this.sessionDir(params.workspacePath, params.personaId, params.branchName);
    await mkdir(dir, { recursive: true });

    const uuid = randomUUID();
    const filePath = join(dir, `${uuid}.jsonl`);
    const header = JSON.stringify({ _meta: "header", startedAt: Date.now() });
    await writeFile(filePath, header + "\n", "utf-8");

    this._pendingWrites.set(filePath, Promise.resolve());
    return { uuid, filePath };
  }

  /** Pending write chains per file path — guarantees append ordering. */
  private _pendingWrites = new Map<string, Promise<void>>();

  /** Append a log entry. Non-blocking but ordered via promise chaining. */
  append(handle: SessionHandle, entry: PersistLogEntry): void {
    const line = JSON.stringify(entry) + "\n";
    const prev = this._pendingWrites.get(handle.filePath) ?? Promise.resolve();
    const next = prev.then(() => appendFile(handle.filePath, line, "utf-8")).catch((err) => {
      console.error(`[session-persistence] append failed: ${err}`);
    });
    this._pendingWrites.set(handle.filePath, next);
  }

  /** Close a session. Awaits pending appends, then writes footer. */
  async close(handle: SessionHandle): Promise<void> {
    await this._pendingWrites.get(handle.filePath);
    const footer = JSON.stringify({ _meta: "footer", endedAt: Date.now() });
    await appendFile(handle.filePath, footer + "\n", "utf-8");
    this._pendingWrites.delete(handle.filePath);
  }
}

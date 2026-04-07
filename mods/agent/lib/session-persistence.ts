import { mkdir, writeFile, appendFile, readdir, readFile } from "node:fs/promises";
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

  /** List sessions in the scoped directory, sorted by startedAt desc. */
  async list(workspacePath: string, personaId: string, branchName: string): Promise<SessionMeta[]> {
    const dir = this.sessionDir(workspacePath, personaId, branchName);
    let files: string[];
    try {
      files = (await readdir(dir)).filter((f) => f.endsWith(".jsonl"));
    } catch {
      return [];
    }

    const metas: SessionMeta[] = [];
    for (const file of files) {
      const meta = await this.readSessionMeta(join(dir, file));
      if (meta) metas.push(meta);
    }

    return metas.sort((a, b) => b.startedAt - a.startedAt);
  }

  /** Read a full session log, returning entries (skipping _meta lines). */
  async read(
    workspacePath: string,
    personaId: string,
    branchName: string,
    uuid: string,
  ): Promise<PersistLogEntry[]> {
    const filePath = join(this.sessionDir(workspacePath, personaId, branchName), `${uuid}.jsonl`);
    let content: string;
    try {
      content = await readFile(filePath, "utf-8");
    } catch {
      return [];
    }
    return parseLogEntries(content);
  }

  /** Find the UUID of the most recent session. */
  async findLatestSessionUuid(
    workspacePath: string,
    personaId: string,
    branchName: string,
  ): Promise<string | null> {
    const sessions = await this.list(workspacePath, personaId, branchName);
    return sessions.length > 0 ? sessions[0].uuid : null;
  }

  /** Read header + scan for first user message as preview. */
  private async readSessionMeta(filePath: string): Promise<SessionMeta | null> {
    let content: string;
    try {
      content = await readFile(filePath, "utf-8");
    } catch {
      return null;
    }

    const lines = content.split("\n").filter((l) => l.trim());
    if (lines.length === 0) return null;

    let header: { _meta: string; startedAt: number };
    try {
      header = JSON.parse(lines[0]);
    } catch {
      return null;
    }
    if (header._meta !== "header" || typeof header.startedAt !== "number") return null;

    const uuid = filePath.split("/").pop()!.replace(".jsonl", "");

    let preview: string | null = null;
    for (let i = 1; i < Math.min(lines.length, 10); i++) {
      try {
        const entry = JSON.parse(lines[i]);
        if (entry.type === "user" && entry.message) {
          preview = entry.message.length > 100
            ? entry.message.slice(0, 100) + "\u2026"
            : entry.message;
          break;
        }
      } catch {
        continue;
      }
    }

    return { uuid, startedAt: header.startedAt, preview };
  }
}

function parseLogEntries(content: string): PersistLogEntry[] {
  const entries: PersistLogEntry[] = [];
  for (const line of content.split("\n")) {
    if (!line.trim()) continue;
    try {
      const parsed = JSON.parse(line);
      if (parsed._meta) continue;
      entries.push(parsed as PersistLogEntry);
    } catch {
      continue;
    }
  }
  return entries;
}

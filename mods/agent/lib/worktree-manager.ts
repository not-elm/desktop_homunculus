import { join } from "node:path";
import { gitExec } from "./git.ts";

/** Information about a git worktree. */
export interface WorktreeInfo {
  name: string;
  path: string;
  branch: string;
  baseBranch: string;
  repoDir: string;
}

/** Diff statistics for a worktree. */
export interface WorktreeStatus {
  commits: number;
  filesChanged: number;
  insertions: number;
  deletions: number;
  hasUncommittedChanges: boolean;
  canMerge: boolean;
}

/** Result of a merge operation. */
export interface MergeResult {
  success: boolean;
  error?: string;
}

const WORKTREE_DIR = ".hmcs/worktrees";

/** Manages git worktrees within a repository. */
export class WorktreeManager {
  constructor(private readonly repoDir: string) {}

  /** Create a new worktree branching from the given base branch. */
  async create(name: string, baseBranch: string): Promise<WorktreeInfo> {
    const worktreePath = this.worktreePath(name);
    await gitExec(this.repoDir, [
      "worktree",
      "add",
      "-b",
      name,
      worktreePath,
      baseBranch,
    ]);
    return {
      name,
      path: worktreePath,
      branch: name,
      baseBranch,
      repoDir: this.repoDir,
    };
  }

  /** Remove a worktree by name. */
  async remove(name: string): Promise<void> {
    const worktreePath = this.worktreePath(name);
    await gitExec(this.repoDir, [
      "worktree",
      "remove",
      "--force",
      worktreePath,
    ]);
    await this.deleteBranchSafely(name);
  }

  /** Fast-forward merge a worktree's branch into its base branch, then remove. */
  async merge(name: string): Promise<MergeResult> {
    const info = await this.findByName(name);
    if (!info) return { success: false, error: `Worktree "${name}" not found` };

    try {
      await gitExec(this.repoDir, ["merge", "--ff-only", info.branch]);
      await this.remove(name);
      return { success: true };
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      return { success: false, error: message };
    }
  }

  /** List all managed worktrees (excludes the main worktree). */
  async list(): Promise<WorktreeInfo[]> {
    const raw = await gitExec(this.repoDir, [
      "worktree",
      "list",
      "--porcelain",
    ]);
    return this.parseManagedWorktrees(raw);
  }

  /** Get diff statistics for a worktree relative to its base branch. */
  async status(name: string): Promise<WorktreeStatus> {
    const info = await this.findByName(name);
    if (!info) throw new Error(`Worktree "${name}" not found`);

    const [commits, diffStat, uncommitted, canMerge] = await Promise.all([
      this.countCommits(info),
      this.diffStat(info),
      this.hasUncommittedChanges(name),
      this.canFastForward(info),
    ]);

    return { commits, ...diffStat, hasUncommittedChanges: uncommitted, canMerge };
  }

  /** Check if a worktree has uncommitted changes. */
  async hasUncommittedChanges(name: string): Promise<boolean> {
    const worktreePath = this.worktreePath(name);
    const result = await gitExec(worktreePath, ["status", "--porcelain"]);
    return result.trim().length > 0;
  }

  private worktreePath(name: string): string {
    return join(this.repoDir, WORKTREE_DIR, name);
  }

  private async findByName(name: string): Promise<WorktreeInfo | null> {
    const all = await this.list();
    return all.find((w) => w.name === name) ?? null;
  }

  private async countCommits(info: WorktreeInfo): Promise<number> {
    try {
      const result = await gitExec(info.path, [
        "rev-list",
        "--count",
        `${info.baseBranch}..${info.branch}`,
      ]);
      return parseInt(result.trim(), 10) || 0;
    } catch {
      return 0;
    }
  }

  private async diffStat(
    info: WorktreeInfo,
  ): Promise<{ filesChanged: number; insertions: number; deletions: number }> {
    try {
      const result = await gitExec(info.path, [
        "diff",
        "--shortstat",
        `${info.baseBranch}...${info.branch}`,
      ]);
      return parseDiffShortstat(result);
    } catch {
      return { filesChanged: 0, insertions: 0, deletions: 0 };
    }
  }

  private async canFastForward(info: WorktreeInfo): Promise<boolean> {
    try {
      const mergeBase = (
        await gitExec(this.repoDir, [
          "merge-base",
          info.baseBranch,
          info.branch,
        ])
      ).trim();
      const baseHead = (
        await gitExec(this.repoDir, ["rev-parse", info.baseBranch])
      ).trim();
      return mergeBase === baseHead;
    } catch {
      return false;
    }
  }

  private async deleteBranchSafely(branchName: string): Promise<void> {
    try {
      await gitExec(this.repoDir, ["branch", "-D", branchName]);
    } catch {
      // Branch may already be deleted or is current -- ignore
    }
  }

  private parseManagedWorktrees(porcelainOutput: string): WorktreeInfo[] {
    const entries = porcelainOutput.trim().split("\n\n");
    const managedPrefix = normalizePath(
      join(this.repoDir, WORKTREE_DIR),
    );

    return entries
      .map((entry) => this.parseWorktreeEntry(entry, managedPrefix))
      .filter((info): info is WorktreeInfo => info !== null);
  }

  private parseWorktreeEntry(
    entry: string,
    managedPrefix: string,
  ): WorktreeInfo | null {
    const lines = entry.split("\n");
    const pathLine = lines.find((l) => l.startsWith("worktree "));
    const branchLine = lines.find((l) => l.startsWith("branch "));
    if (!pathLine || !branchLine) return null;

    const wtPath = normalizePath(pathLine.replace("worktree ", ""));
    if (!wtPath.startsWith(managedPrefix)) return null;

    const branch = branchLine.replace("branch refs/heads/", "");
    const name = wtPath.split("/").pop() ?? branch;

    return {
      name,
      path: wtPath,
      branch,
      baseBranch: "main", // Stored separately in preferences; default fallback
      repoDir: this.repoDir,
    };
  }
}

/** Normalize a file path to use forward slashes for cross-platform comparison. */
function normalizePath(p: string): string {
  return p.replace(/\\/g, "/");
}

/** Parse git diff --shortstat output into structured numbers. */
function parseDiffShortstat(stat: string): {
  filesChanged: number;
  insertions: number;
  deletions: number;
} {
  const filesMatch = stat.match(/(\d+) files? changed/);
  const insertMatch = stat.match(/(\d+) insertions?/);
  const deleteMatch = stat.match(/(\d+) deletions?/);
  return {
    filesChanged: filesMatch ? parseInt(filesMatch[1], 10) : 0,
    insertions: insertMatch ? parseInt(insertMatch[1], 10) : 0,
    deletions: deleteMatch ? parseInt(deleteMatch[1], 10) : 0,
  };
}

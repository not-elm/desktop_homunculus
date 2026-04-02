import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

/** Execute a git command in the given directory. */
export async function gitExec(cwd: string, args: string[]): Promise<string> {
  const { stdout } = await execFileAsync("git", args, {
    cwd,
    maxBuffer: 10 * 1024 * 1024,
  });
  return stdout;
}

/** Check if a directory is inside a git repository. */
export async function isGitRepo(dir: string): Promise<boolean> {
  try {
    const result = await gitExec(dir, [
      "rev-parse",
      "--is-inside-work-tree",
    ]);
    return result.trim() === "true";
  } catch {
    return false;
  }
}

/** Get the current branch name. Returns null if in detached HEAD. */
export async function currentBranch(cwd: string): Promise<string | null> {
  try {
    const result = await gitExec(cwd, ["rev-parse", "--abbrev-ref", "HEAD"]);
    const branch = result.trim();
    return branch === "HEAD" ? null : branch;
  } catch {
    return null;
  }
}

/** List all local branch names. */
export async function listBranches(cwd: string): Promise<string[]> {
  const result = await gitExec(cwd, [
    "branch",
    "--format=%(refname:short)",
  ]);
  return result.trim().split("\n").filter(Boolean);
}

/** Get the git toplevel directory for a path. */
export async function gitToplevel(cwd: string): Promise<string> {
  const result = await gitExec(cwd, ["rev-parse", "--show-toplevel"]);
  return result.trim();
}

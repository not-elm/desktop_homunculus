import { spawn } from "node:child_process";
import { resolve as resolvePath } from "node:path";

/** Strip the `\\?\` verbatim path prefix that Windows file dialogs may return. */
function normalizeWindowsCwd(cwd: string): string {
  return resolvePath(cwd.replace(/^\\\\\?\\/, ""));
}

/** Execute a git command in the given directory. */
export function gitExec(cwd: string, args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    const child = spawn("git", args, {
      cwd: normalizeWindowsCwd(cwd),
      shell: false,
      windowsHide: true,
    });

    const chunks: Buffer[] = [];
    const errChunks: Buffer[] = [];

    child.stdout.on("data", (d: Buffer) => chunks.push(d));
    child.stderr.on("data", (d: Buffer) => errChunks.push(d));
    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) {
        resolve(Buffer.concat(chunks).toString());
      } else {
        reject(new Error(Buffer.concat(errChunks).toString() || `git exited with code ${code}`));
      }
    });
  });
}

/** Check if a directory is inside a git repository. */
export async function isGitRepo(dir: string): Promise<boolean> {
  try {
    const result = await gitExec(dir, ["rev-parse", "--is-inside-work-tree"]);
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
  const result = await gitExec(cwd, ["branch", "--format=%(refname:short)"]);
  return result.trim().split("\n").filter(Boolean);
}

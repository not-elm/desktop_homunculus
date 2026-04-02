import { describe, it, expect, beforeEach, afterEach } from "vitest";
import { mkdtempSync, rmSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { execSync } from "node:child_process";
import { WorktreeManager } from "./worktree-manager.ts";

function createTempGitRepo(): string {
  const dir = mkdtempSync(join(tmpdir(), "wt-test-"));
  execSync("git init", { cwd: dir });
  execSync("git config user.email test@test.com", { cwd: dir });
  execSync("git config user.name Test", { cwd: dir });
  writeFileSync(join(dir, "README.md"), "# Test");
  execSync("git add . && git commit -m init", { cwd: dir });
  return dir;
}

describe("WorktreeManager", () => {
  let repoDir: string;
  let manager: WorktreeManager;

  beforeEach(() => {
    repoDir = createTempGitRepo();
    manager = new WorktreeManager(repoDir);
  });
  afterEach(() => {
    rmSync(repoDir, { recursive: true, force: true });
  });

  describe("create", () => {
    it("creates a worktree with a new branch", async () => {
      const info = await manager.create("my-feature", "main");
      expect(info.name).toBe("my-feature");
      expect(info.branch).toBe("my-feature");
      expect(existsSync(info.path)).toBe(true);
      expect(existsSync(join(info.path, "README.md"))).toBe(true);
    });

    it("throws if name already exists", async () => {
      await manager.create("dup", "main");
      await expect(manager.create("dup", "main")).rejects.toThrow();
    });
  });

  describe("list", () => {
    it("returns all worktrees under the repo", async () => {
      await manager.create("wt-a", "main");
      await manager.create("wt-b", "main");
      const list = await manager.list();
      const names = list.map((w) => w.name);
      expect(names).toContain("wt-a");
      expect(names).toContain("wt-b");
    });
  });

  describe("remove", () => {
    it("removes a worktree", async () => {
      const info = await manager.create("to-remove", "main");
      await manager.remove("to-remove");
      expect(existsSync(info.path)).toBe(false);
    });
  });

  describe("status", () => {
    it("returns diff stats for a worktree", async () => {
      const info = await manager.create("with-changes", "main");
      writeFileSync(join(info.path, "new-file.ts"), "export const x = 1;");
      execSync('git add . && git commit -m "add file"', { cwd: info.path });
      const status = await manager.status("with-changes");
      expect(status.commits).toBe(1);
      expect(status.filesChanged).toBeGreaterThan(0);
    });
  });

  describe("merge", () => {
    it("fast-forward merges worktree branch into base", async () => {
      const info = await manager.create("to-merge", "main");
      writeFileSync(join(info.path, "feature.ts"), "export const y = 2;");
      execSync('git add . && git commit -m "add feature"', { cwd: info.path });
      const result = await manager.merge("to-merge");
      expect(result.success).toBe(true);
      expect(existsSync(join(repoDir, "feature.ts"))).toBe(true);
    });

    it("returns failure on non-fast-forward merge", async () => {
      const info = await manager.create("diverge", "main");
      writeFileSync(join(repoDir, "main-change.ts"), "main");
      execSync('git add main-change.ts && git commit -m "main change"', {
        cwd: repoDir,
      });
      writeFileSync(join(info.path, "wt-change.ts"), "wt");
      execSync('git add . && git commit -m "wt change"', { cwd: info.path });
      const result = await manager.merge("diverge");
      expect(result.success).toBe(false);
    });
  });

  describe("hasUncommittedChanges", () => {
    it("returns false for clean worktree", async () => {
      await manager.create("clean", "main");
      expect(await manager.hasUncommittedChanges("clean")).toBe(false);
    });

    it("returns true for dirty worktree", async () => {
      const info = await manager.create("dirty", "main");
      writeFileSync(join(info.path, "uncommitted.ts"), "dirty");
      expect(await manager.hasUncommittedChanges("dirty")).toBe(true);
    });
  });
});

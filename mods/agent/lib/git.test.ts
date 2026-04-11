import { execSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { currentBranch, gitExec, isGitRepo, listBranches } from './git.ts';

function createTempGitRepo(): string {
  const dir = mkdtempSync(join(tmpdir(), 'git-test-'));
  execSync('git init', { cwd: dir });
  execSync('git config user.email test@test.com', { cwd: dir });
  execSync('git config user.name Test', { cwd: dir });
  writeFileSync(join(dir, 'README.md'), '# Test');
  execSync('git add . && git commit -m init', { cwd: dir });
  return dir;
}

describe('gitExec', () => {
  let repoDir: string;
  beforeEach(() => {
    repoDir = createTempGitRepo();
  });
  afterEach(() => {
    rmSync(repoDir, { recursive: true, force: true });
  });

  it('executes a git command and returns stdout', async () => {
    const result = await gitExec(repoDir, ['rev-parse', '--is-inside-work-tree']);
    expect(result.trim()).toBe('true');
  });

  it('throws on invalid git command', async () => {
    await expect(gitExec(repoDir, ['invalid-command'])).rejects.toThrow();
  });
});

describe('isGitRepo', () => {
  let repoDir: string;
  beforeEach(() => {
    repoDir = createTempGitRepo();
  });
  afterEach(() => {
    rmSync(repoDir, { recursive: true, force: true });
  });

  it('returns true for a git repo', async () => {
    expect(await isGitRepo(repoDir)).toBe(true);
  });

  it('returns false for a non-git directory', async () => {
    const nonGit = mkdtempSync(join(tmpdir(), 'non-git-'));
    expect(await isGitRepo(nonGit)).toBe(false);
    rmSync(nonGit, { recursive: true, force: true });
  });
});

describe('currentBranch', () => {
  let repoDir: string;
  beforeEach(() => {
    repoDir = createTempGitRepo();
  });
  afterEach(() => {
    rmSync(repoDir, { recursive: true, force: true });
  });

  it('returns the current branch name', async () => {
    const branch = await currentBranch(repoDir);
    expect(['main', 'master']).toContain(branch);
  });
});

describe('listBranches', () => {
  let repoDir: string;
  beforeEach(() => {
    repoDir = createTempGitRepo();
    execSync('git branch feature-a', { cwd: repoDir });
    execSync('git branch feature-b', { cwd: repoDir });
  });
  afterEach(() => {
    rmSync(repoDir, { recursive: true, force: true });
  });

  it('returns all local branches', async () => {
    const branches = await listBranches(repoDir);
    expect(branches).toContain('feature-a');
    expect(branches).toContain('feature-b');
    expect(branches.length).toBeGreaterThanOrEqual(3);
  });
});

# Design: `create-pr` Skill

> Date: 2026-03-16
> Status: Draft

## Overview

A Claude Code skill that automates PR creation using the project's PR template (`.github/pull_request_template.md`). Invoked manually via `/create-pr`. The skill validates that the diff is consistent with the user's stated problem, generates a full English PR draft, and executes `gh pr create` (or `gh pr edit` for existing PRs) upon approval.

## Placement

```
.claude/skills/create-pr/SKILL.md
```

Single-file skill. No sub-agents or helper files.

## Flow

```
Step 1: Preflight  ──→  failure → abort (show reason)
         │
    pass ↓
Step 2: Problem input
         │
         ↓
Step 3: Diff retrieval + plausibility check  ──→  mismatch → revise Problem or abort
         │
    pass ↓
Step 4: Full PR draft generation + approval (title, Problem, Solution, checklist — all at once)
         │
  approved ↓
Step 5: push + gh pr create/edit → report URL
```

## Step Details

### Step 1 — Preflight

Run the following checks in order. Abort with a clear message on first failure:

1. **Git repository**: Confirm the working directory is a git repo.
2. **Not detached HEAD**: `git symbolic-ref HEAD` must succeed.
3. **Resolve base branch**: Run `git symbolic-ref refs/remotes/origin/HEAD` to determine the default branch (e.g., `origin/main`). Strip the remote prefix to get the local name (e.g., `main`).
4. **Not on base branch**: Current branch (`git branch --show-current`) must differ from the base branch.
5. **Commits exist**: `git rev-list --count <base>...HEAD` must be > 0.
6. **Clean working tree**: `git status --porcelain` must be empty. Abort if uncommitted changes exist.
7. **`gh` authenticated**: `gh auth status` must succeed. On failure, instruct the user to run `gh auth login`.
8. **Existing PR check**: Run `gh pr view --json url 2>/dev/null`. If a PR exists for the current branch, switch to **update mode** (use `gh pr edit` in Step 5 instead of `gh pr create`). Inform the user which mode is active.

### Step 2 — Problem Input

Ask the user:

> What problem does this PR solve? (Any language is fine — the final PR will be in English.)
> Optionally include related issue numbers (e.g., #123).

Accept free-form input in any language that Claude Code can process. Store the raw input and any issue references separately.

### Step 3 — Diff Retrieval + Plausibility Check

1. Retrieve the diff: `git diff <base>...HEAD`
2. Retrieve commit history: `git log <base>...HEAD --oneline`
3. Perform a **plausibility check**: assess whether the diff is consistent with the user's stated Problem. This is NOT a proof of resolution — it checks whether the changes plausibly address what the user described.
   - For `feat`/`fix` type changes: verify the diff touches relevant code areas.
   - For `docs`/`refactor`/`chore`/`test`/`ci`/`build` type changes: verify the diff is relevant to the stated goal (e.g., a docs Problem should correspond to documentation file changes).

**If consistent**: Proceed to Step 4.

**If inconsistent**: Explain the mismatch and present options:
- (a) Revise the Problem description → return to Step 2
- (b) Abort PR creation

### Step 4 — Full PR Draft Generation + Approval

Generate all PR components in English from the diff, commit history, and user's Problem input. Present the complete PR as a single block for review:

**Title**: Conventional commit format. Select the prefix (`feat`, `fix`, `docs`, `refactor`, `chore`, `test`, `ci`, `build`) based on the dominant change type. Use imperative mood. Keep it under 70 characters.

**Body** (using the template structure):

```markdown
## Problem

{User's problem, translated/refined into English. Include `Closes #N` if issue numbers were provided.}

## Solution

{What changed, why this approach, which parts of the codebase are affected (engine, packages, mods, docs).}
{If breaking change detected: include a `### Breaking Changes` subsection.}

---

- [x/​ ] If HTTP endpoints changed: I ran `make gen-open-api` and `pnpm build`
- [x/​ ] This PR includes breaking changes
```

**Checklist auto-detection**:
- **HTTP endpoint changes**: Check if diff touches `engine/crates/homunculus_http_server/src/**`. If yes, mark checked.
- **Breaking changes**: Check for removed/renamed public APIs, changed HTTP response shapes, removed config keys, etc. If detected, mark checked and generate `### Breaking Changes` subsection in Solution.
- **UI changes**: If diff touches `mods/*/ui/**` or `packages/ui/**`, remind the user to add screenshots (but do not block).

Present the full draft and ask the user to approve or request edits. If edits are requested, regenerate and re-present. Repeat until approved.

### Step 5 — Push + PR Create/Edit

1. Push the branch: `git push -u origin <current-branch>`
2. Based on Preflight result:
   - **No existing PR**: `gh pr create --title "<title>" --body "<body>"`
   - **Existing PR**: `gh pr edit --title "<title>" --body "<body>"`
3. Report the PR URL to the user.

## Constraints

- All PR output (title and body) MUST be in English regardless of input language.
- The skill MUST NOT proceed past a failed preflight check.
- The skill MUST NOT create a PR without explicit user approval of the full draft.
- The skill does NOT run tests — it only performs code-review-style plausibility checking.
- The skill reads `.github/pull_request_template.md` as the canonical template structure. If the template is missing, abort with a message.

## Design Decisions

| Decision | Rationale |
|----------|-----------|
| Single-file SKILL.md | Complexity is manageable without sub-agents. Easier to maintain. |
| Plausibility check, not proof | Semantic "proof of resolution" is infeasible for docs/refactor/chore PRs. Plausibility is achievable and useful. |
| Mismatch → revise or abort (not hard abort) | User may have described the problem poorly. Allow revision before giving up. |
| One approval step (not two) | Avoids redundancy and the inconsistency of editing after partial approval. |
| Dynamic base branch resolution | Avoids hardcoding `main`. Works with any default branch. |
| Preflight before any user input | Catches objective failures (no commits, wrong branch, auth issues) before wasting user's time. |
| Existing PR → update mode | Common case when iterating on a PR. Better UX than aborting. |

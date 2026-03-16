---
name: create-pr
description: >
  Create or update a GitHub pull request using the project's PR template.
  Use when the user wants to open a PR for the current branch.
---

# Create Pull Request

Create or update a GitHub PR using the project's PR template. Validates that the diff is consistent with the stated problem, generates a full English PR draft, and executes `gh pr create` (or `gh pr edit`) upon approval.

## Flow

```
Step 1: Preflight  ──→  failure → abort (show reason)
         │
    pass ↓
Step 2: Problem input
         │
         ↓
Step 3: Diff retrieval + plausibility check  ──→  mismatch → revise or abort
         │
    pass ↓
Step 4: Full PR draft generation + approval
         │
  approved ↓
Step 4.5: Label selection (AskUserQuestion multiSelect)
         │
         ↓
Step 5: push + gh pr create/edit → report URL
```

## Steps

### 1. Preflight

Run these checks in order. Abort with a clear message on first failure:

1. **Git repository**: Confirm the working directory is a git repo (`git rev-parse --git-dir`).
2. **Not detached HEAD**: `git symbolic-ref HEAD` must succeed.
3. **Resolve base branch**: Run `git symbolic-ref refs/remotes/origin/HEAD` and strip the remote prefix (e.g., `refs/remotes/origin/main` → `main`). If this fails, fall back to checking `git rev-parse --verify origin/main` or `origin/master`. If neither exists, ask the user to specify the base branch.
4. **Not on base branch**: `git branch --show-current` must differ from the resolved base branch.
5. **Commits exist**: `git rev-list --count origin/<base>...HEAD` must be > 0.
6. **Clean working tree**: `git status --porcelain` must produce no output. Abort if uncommitted changes exist.
7. **`gh` authenticated**: `gh auth status` must succeed. On failure, tell the user to run `gh auth login`.
8. **Existing PR check**: Run `gh pr view --json url 2>/dev/null`.
   - If a PR exists: switch to **update mode** and inform the user: "Existing PR found: <url>. Will update it."
   - If no PR exists: use **create mode**.

### 2. Problem Input

Ask the user:

> What problem does this PR solve? (Any language is fine — the final PR will be in English.)
> Optionally include related issue numbers (e.g., #123).

Accept free-form input in any language. Extract and store issue references (e.g., `#123`) separately from the problem description.

### 3. Diff Retrieval + Plausibility Check

1. Run `git diff origin/<base>...HEAD`. If the diff is too large for context, use `git diff origin/<base>...HEAD --stat` for an overview and selectively read key changed files. For very large branches, rely more on the commit history.
2. Run `git log origin/<base>...HEAD --oneline` for commit history.
3. Perform a **plausibility check** — assess whether the diff is consistent with the user's stated Problem:
   - For `feat`/`fix` type changes: verify the diff touches relevant code areas.
   - For `docs`/`refactor`/`chore`/`test`/`ci`/`build` type changes: verify the diff is relevant to the stated goal.

   This is a plausibility check, NOT a proof of resolution.

**If consistent**: Proceed to Step 4.

**If inconsistent**: Explain the mismatch, then present options:
- (a) Revise the Problem description → return to Step 2
- (b) Abort PR creation

### 4. Full PR Draft Generation + Approval

Read `.github/pull_request_template.md` to obtain the current template structure. Generate all components in English:

**Title**: Conventional commit format. Select the prefix (`feat`, `fix`, `docs`, `refactor`, `chore`, `test`, `ci`, `build`) based on the dominant change type. Use imperative mood. Under 70 characters.

**Body**: Follow the template structure:

```
## Problem

{User's problem, translated/refined into English. Include `Closes #N` if issue numbers were provided.}

## Solution

{What changed, why this approach, which parts of the codebase are affected (engine, packages, mods, docs).}

---

- [x] or [ ] If HTTP endpoints changed: I ran `make gen-open-api` and `pnpm build`
- [x] or [ ] This PR includes breaking changes
```

**Checklist auto-detection**:
- **HTTP endpoint changes**: If diff touches `engine/crates/homunculus_http_server/src/**`, mark `[x]`. Otherwise `[ ]`.
- **Breaking changes**: If removed/renamed public APIs, changed HTTP response shapes, or removed config keys detected, mark `[x]` and add a `### Breaking Changes` subsection in Solution. Otherwise `[ ]`.
- **UI changes**: If diff touches `mods/*/ui/**` or `packages/ui/**`, remind the user to consider adding screenshots.

Present the complete PR draft (title + body) and ask the user to approve or request edits. If edits are requested, apply them and re-present. Repeat until approved.

### 4.5. Label Selection

After the user approves the PR draft, present available labels for selection:

1. **Fetch available labels**: Run `gh label list --json name,description --limit 100` to get all repo labels dynamically. If the command fails or returns no labels, skip this step entirely and proceed to Step 5.

2. **Split into groups of up to 4**: Partition the labels into groups of 4. Each group becomes one `question` in the `AskUserQuestion` call.

3. **Present via `AskUserQuestion`**: Use `multiSelect: true` on each question. Format each question title as `"Labels (N/M)"` to indicate pagination (e.g., `"Labels (1/3)"`, `"Labels (2/3)"`). Use the label `name` as the option `label` and the label `description` as the option `description` (if description is empty, omit it).
   - If total labels ≤ 16: use a single `AskUserQuestion` call with up to 4 questions.
   - If total labels > 16: use multiple `AskUserQuestion` calls sequentially.

4. **Collect results**: Merge all selected labels. If the user selects nothing, proceed without labels.

### 5. Push + PR Create/Edit

1. Push the branch: `git push -u origin <current-branch>`. If push fails, report the error and abort. Do NOT use `--force` unless the user explicitly requests it.
2. Create or update the PR:
   - **Create mode**: `gh pr create --base <base-branch> --title "<title>" --body "<body>"` — append `-l <label>` for each selected label (e.g., `gh pr create ... -l bug -l enhancement`).
   - **Update mode**: `gh pr edit --title "<title>" --body "<body>"` — append `--add-label <label>` for each selected label (e.g., `gh pr edit ... --add-label bug --add-label enhancement`).
   - If no labels were selected, omit the label flags entirely.
3. Report the PR URL to the user.

## Constraints

- All PR output (title and body) MUST be in English regardless of input language.
- MUST NOT proceed past a failed preflight check.
- MUST NOT create a PR without explicit user approval of the full draft.
- Does NOT run tests — plausibility checking only.
- If `.github/pull_request_template.md` is missing, abort with a message.

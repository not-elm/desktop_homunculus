---
name: create-issue
description: >
  Create a GitHub proposal issue with spec-level brainstorming.
  Use when the user wants to file a new proposal (enhancement) issue.
---

# Create Issue

Create a GitHub proposal issue. Takes a free-form idea (any language), structures it into the project's proposal template fields, checks for duplicates, refines the spec via brainstorming, and files the issue on GitHub. All output is in English.

## Flow

```
/create-issue <proposal description in any language>
  |
  v
1. Argument parsing
  |  - Extract proposal content from args
  |  - No args → stop
  |
  v
2. Preflight
  |  - git repo check
  |  - gh auth check
  |  - Failure → abort with message
  |
  v
3. Initial structuring
  |  - Best-effort mapping to 4-5 template fields + title
  |  - All English output
  |
  v
4. Duplicate detection (two-stage)
  |  - Stage 1: keyword search from initial structure
  |  - Stage 2: semantic check on top 5 (if Stage 1 has hits)
  |  - Likely duplicate / related found → present, ask to continue or abort
  |  - Command failure → warn, skip to step 5
  |
  v
5. Spec-level brainstorming (delegate to superpowers:brainstorming)
  |  - Refine problem, solution, scope
  |  - NOT implementation design
  |
  v
6. Final draft (build issue from refined spec) → approve / edit / abort
  |
  v
7. gh issue create (HEREDOC body, enhancement label) → report URL
  |  - Hint: "Run /brainstorm-issue #<number> to start designing a solution."
```

## Argument Parsing

| Input | Action |
|-------|--------|
| Args provided | Use as proposal content, any language |
| (none) | Display "Please provide a proposal description." and stop |

## Preflight

Run checks in order. Abort on first failure:

| Check | Command | Failure |
|-------|---------|---------|
| Git repository | `git rev-parse --git-dir` | Abort: "Not a git repository." |
| `gh` authenticated | `gh auth status` | Abort: "Not authenticated. Run `gh auth login`." |

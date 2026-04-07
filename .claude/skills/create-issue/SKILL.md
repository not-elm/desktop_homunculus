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

## Initial Structuring

Take the user's free-form input (any language) and produce a best-effort English draft.

**Title**: Generate a concise English title. Priority: meaning preservation > imperative mood > under 80 characters.

**Body fields**:

| Field | Required | Output format |
|-------|----------|---------------|
| `## Type` | Yes | Exact literal: `New feature` or `Improvement to existing feature` |
| `## Problem / Current Behavior` | Yes | Free-form English prose |
| `## Proposed Solution` | Yes | Free-form English prose |
| `## Affected Area` | Yes | Multi-select, slash-separated from: `engine / packages / mods / docs / website / ci/build / other` |
| `## Alternatives Considered` | No | Include if user mentioned alternatives. Otherwise omit section entirely |

### Rules

- Do NOT include boilerplate checklists (`- [ ] I searched existing issues...`).
- Do NOT include HTML comments.
- If input is too vague to fill a required field meaningfully, produce a best-effort draft. The brainstorming phase (step 5) will refine it.

This initial structure serves two purposes:

1. Provides keywords for duplicate detection (step 4)
2. Provides starting material for the brainstorming delegation (step 5)

## Duplicate Detection

### Stage 1 — Keyword Search

Extract 3-5 key terms from the initial structure (English). Run:

```
gh issue list --search "<keywords> is:open" --json number,title,url,labels --limit 10
```

| Result | Action |
|--------|--------|
| 0 hits | Skip Stage 2, proceed to brainstorming (step 5) |
| 1+ hits | Proceed to Stage 2 |
| Command failure | Warn ("Duplicate check failed, skipping"), proceed to brainstorming (step 5) |

### Stage 2 — Semantic Check

Take the first 5 results (search-rank order as returned by `gh`). Fetch each:

```
gh issue view <number> --json title,body
```

Classify each via LLM judgment:

- **Likely duplicate** — same problem, similar solution
- **Related** — overlapping area, different problem or solution
- **Not duplicate** — superficial keyword match only

| Result | Action |
|--------|--------|
| Likely duplicate or related found | Present matches, ask to continue or abort |
| Only "not duplicate" | Proceed silently |
| Individual fetch failure | Skip that candidate, continue with remaining |

### Presentation Format

If likely duplicate or related issues are found, present them:

```
### Potential Duplicates Found

**Likely duplicate:**
- #42 — Add character switcher to tray menu

**Related:**
- #87 — System tray integration improvements

Continue creating this issue? (yes / no)
```

User declines → stop. User continues → proceed to brainstorming (step 5).

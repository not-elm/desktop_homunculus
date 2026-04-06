---
name: brainstorm-issue
description: Use when starting work on a GitHub Issue to fetch issue content and begin brainstorming. Triggers on issue URL or number input.
---

# Brainstorm Issue

Fetch a GitHub Issue (proposal template), structure its content, and delegate to `superpowers:brainstorming` for design work. This skill handles Issue retrieval and structuring only. All design decisions, code generation, and implementation are the responsibility of the brainstorming skill.

## Flow

```
User: /brainstorm-issue #123 (or URL)
  |
  v
1. Parse argument (URL or issue number)
  |
  v
2. Fetch issue via gh issue view
  |
  v
3. Check labels for proposal detection
  |  - enhancement label → continue
  |  - bug / documentation label → warn + ask to confirm
  |  - no label → continue without warning
  |
  v
4. Extract and structure proposal template fields
  |
  v
5. Invoke superpowers:brainstorming with structured context
```

## Argument Parsing

Parse the skill argument to determine the issue number and repository.

**Input formats:**

| Input | Interpretation |
|-------|---------------|
| `123` | Current repo Issue #123 |
| `#123` | Same as above (`#` prefix allowed) |
| `https://github.com/owner/repo/issues/123` | Extract owner/repo and number from URL |

**Parsing steps:**

1. If no argument is provided, display "Issue番号またはURLを指定してください" and stop.
2. If the argument starts with `https://github.com/`:
   - Extract `owner/repo` and the issue number from the URL path.
   - Use: `gh issue view <number> -R <owner/repo> --json title,labels,body`
3. Otherwise:
   - Strip leading `#` if present and extract the number.
   - Use: `gh issue view <number> --json title,labels,body`
   - This uses `gh`'s default behavior, which resolves the repo from the cwd's git remote origin.

If `gh` returns an error (issue not found, auth failure, CLI not installed), display the error output as-is and stop.

## Non-Proposal Detection

After fetching the issue, check its labels to determine if it was filed using the proposal template.

| Label | Verdict | Action |
|-------|---------|--------|
| `enhancement` | Proposal | Continue |
| `bug` | Non-proposal | Warn: "This issue has the `bug` label and may not be a proposal. Continue anyway?" If the user declines, stop. |
| `documentation` | Non-proposal | Warn: "This issue has the `documentation` label and may not be a proposal. Continue anyway?" If the user declines, stop. |
| No labels | Unknown | Continue without warning |

If the issue has multiple labels, the presence of `bug` or `documentation` triggers the warning regardless of other labels.

## Content Structuring

### Target headings

Extract content under these five `##`-level headings (case-sensitive):

1. `## Type`
2. `## Problem / Current Behavior`
3. `## Proposed Solution`
4. `## Affected Area`
5. `## Alternatives Considered`

### Extraction rules

- For each heading found, capture all text between it and the next `##` heading (or end of body).
- If a heading is not found, omit that section from the output entirely.
- If **none** of the five headings are found (non-template issue), use the full issue body as a single `### Issue Body` section (fallback).
- Remove checklist lines at the top of the body that match the template boilerplate (`- [ ] I searched existing issues...`).
- Remove HTML comments (`<!-- ... -->`).
- Keep all other checklists in the body (they may contain useful information).

### Output format

Assemble the structured markdown block as follows:

```
## Issue Context: #<number> — <title>

**Labels**: <comma-separated labels>
**URL**: <issue URL>

### Type
<extracted content>

### Problem / Current Behavior
<extracted content>

### Proposed Solution
<extracted content>

### Affected Area
<extracted content>

### Alternatives Considered
<extracted content>
```

Omit any `###` section whose heading was not found in the issue body.

Fallback format (when none of the five headings are found):

```
## Issue Context: #<number> — <title>

**Labels**: <comma-separated labels>
**URL**: <issue URL>

### Issue Body
<full issue body>
```

## Delegation to brainstorming

After structuring the issue content, invoke the brainstorming skill:

1. Call the Skill tool with `skill: "superpowers:brainstorming"`.
2. Pass the structured Issue Context block (from the previous step) as the `args` value, prefixed with: "Design a solution for the following GitHub Issue. The issue content is provided below as prior context — use it to inform your brainstorming process rather than re-asking questions the issue already answers."

**Important:**

- Do NOT modify the `superpowers:brainstorming` skill itself.
- The issue context is provided as **prior information**. How brainstorming uses it (skipping questions, asking for deeper detail, etc.) is up to the brainstorming skill's own flow.
- All brainstorming steps (explore project context, clarifying questions, propose approaches, etc.) proceed as normal.

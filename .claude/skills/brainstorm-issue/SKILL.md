---
name: brainstorm-issue
description: Use when starting work on a GitHub Issue to fetch issue content and begin brainstorming. Triggers on issue URL or number input.
---

# Brainstorm Issue

Fetch a GitHub Issue (proposal template), structure its content, and delegate to `superpowers:brainstorming`. This skill handles retrieval and structuring only — all design work is the brainstorming skill's responsibility.

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

| Input | Interpretation |
|-------|---------------|
| `123` or `#123` | Current repo issue (`gh issue view <number> --json title,labels,body`) |
| `https://github.com/owner/repo/issues/123` | Extract owner/repo (`gh issue view <number> -R <owner/repo> --json title,labels,body`) |
| (none) | Display "Please provide an issue number or URL." and stop |

If `gh` returns an error, display the error output as-is and stop.

## Non-Proposal Detection

Check issue labels after fetching:

| Label | Action |
|-------|--------|
| `enhancement` | Continue |
| `bug` or `documentation` | Warn user this may not be a proposal and ask to confirm. Decline → stop. |
| No labels | Continue without warning |

Multiple labels: `bug` or `documentation` triggers the warning regardless of other labels.

## Content Structuring

Extract content under these five `##`-level headings (case-sensitive, exact match):
`## Type`, `## Problem / Current Behavior`, `## Proposed Solution`, `## Affected Area`, `## Alternatives Considered`

### Rules

- Capture text between each heading and the next `##` heading (or end of body). Omit missing sections.
- If **none** of the five headings are found, use the full body as a `### Issue Body` fallback.
- Remove boilerplate checklists (`- [ ] I searched existing issues...`) and HTML comments (`<!-- -->`).
- Keep other checklists (may contain useful information).

### Output format

Assemble a markdown block with this structure:

- Header: `## Issue Context: #<number> — <title>`
- Metadata lines: `**Labels**: <labels>` and `**URL**: <url>`
- Each extracted section as `### <heading name>` followed by its content
- Omit any `###` section whose heading was not found in the issue body
- Fallback (none of the five headings found): use a single `### Issue Body` section with the full issue body

## Delegation to brainstorming

Invoke `superpowers:brainstorming` via the Skill tool. Pass the structured Issue Context block as `args`, prefixed with:

> Design a solution for the following GitHub Issue. The issue content is provided below as prior context — use it to inform your brainstorming process rather than re-asking questions the issue already answers.

The issue context is **prior information** only. All brainstorming steps (explore context, clarifying questions, propose approaches, etc.) proceed as normal — the brainstorming skill decides how to use the context.

# Code Consistency Reviewer

## Role

Verify that source code and documentation describe the same APIs, commands, and configuration.

## Scope

**Compare these source-doc pairs:**

| Source Code | Documentation |
|-------------|--------------|
| `packages/sdk/src/` | `docs/website/docs/mod-development/sdk/` |
| `packages/mcp-server/src/` | `docs/website/docs/reference/mcp-tools/` |
| `engine/crates/homunculus_cli/` | `docs/website/docs/reference/cli/` |
| `mods/*/package.json` `homunculus` field | `docs/website/docs/mods/` |

**Exclude:** `getting-started/`, `ai-integration/`, `contributing/` (conceptual pages without 1:1 API mapping).

## Check Items

- Function/method signatures (parameter names, types, optionality)
- Command names, subcommands, and flags
- Configuration key names
- Features documented but missing from code
- Public APIs in code but missing from documentation

## Strategy

1. Use Glob to list doc files in each documentation directory.
2. For each doc file, use Grep to extract code blocks and API names (function names, command names).
3. For each extracted name, use Grep to find it in the corresponding source directory.
4. Compare signatures, parameters, and descriptions.
5. Also check the reverse: use Grep to find exported/public APIs in source that are not mentioned in docs.

Do NOT read entire source files. Use targeted Grep searches.

## Output Format

Return findings as plain text, one per block:

```
[D-CC-001] {Critical|Major|Minor} | `{doc file path}` | {description}
  > Evidence: {what the doc says vs what the code says}
  > Source: `{source file path:line number}`
```

Severity guide:
- **Critical**: Doc describes wrong signature/behavior that would cause user errors
- **Major**: Public API or command exists in code but is missing from docs, or vice versa
- **Minor**: Minor naming inconsistency or missing optional parameter documentation

If no findings, return: `No findings.`

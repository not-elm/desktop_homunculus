# Unify "MOD commands" Naming

**Date:** 2026-03-17
**Status:** Approved

## Problem

The concept of on-demand scripts exposed by MODs via `"bin"` in `package.json` has no consistent name. Documentation and code comments use multiple terms interchangeably:

- "bin command(s)"
- "on-demand command(s)"
- "on-demand script(s)"
- "bin script(s)"

This inconsistency makes documentation harder to follow and search.

## Decision

Unify all natural-language references to **"MOD command(s)"** (with "MOD" in uppercase, matching existing project convention).

## Scope

### In scope

- Documentation text in `docs/website/` (English and Japanese)
- Project documentation (`CLAUDE.md` files, `engine/CLAUDE.md`, etc.)
- SDK JSDoc comments (`packages/sdk/src/`)
- Rust doc comments (`engine/crates/`)
- File rename: `bin-commands.md` → `commands.md` (English + Japanese)
- Internal Markdown links referencing the renamed file

### Out of scope

- Code identifiers: `executeCommand`, `CommandResult`, `ExecuteCommandRequest`, `CommandEvent`, etc.
- SDK module name: `@hmcs/sdk/commands`
- HTTP API paths: `/commands/execute`
- MCP tool name: `execute_command`
- package.json field: `"bin"`
- Docusaurus URL redirects (not requested)

## Replacement Rules

| Current term | Replacement |
|---|---|
| "bin command(s)" | "MOD command(s)" |
| "on-demand command(s)" | "MOD command(s)" |
| "on-demand script(s)" | "MOD command(s)" |
| "bin script(s)" | "MOD command(s)" |

### Exceptions

- "on-demand" may still be used as a supplementary descriptor when contrasting with services (e.g., "MOD commands are on-demand scripts that run when invoked, unlike services which run continuously.")
- Text inside code blocks, API examples, and command-line examples is not modified.
- Code identifiers are not modified.

## File Rename

- `docs/website/docs/mod-development/bin-commands.md` → `commands.md`
- `docs/website/i18n/ja/docusaurus-plugin-content-docs/current/mod-development/bin-commands.md` → `commands.md`
- Update all internal Markdown links from `bin-commands` to `commands`
- Update Docusaurus sidebar configuration if it references the file name

## Affected Files

### Documentation (~20 files)

- `docs/website/docs/mod-development/bin-commands.md` (rename + content)
- `docs/website/docs/mod-development/project-setup/package-json.md`
- `docs/website/docs/mod-development/project-setup/directory-structure.md`
- `docs/website/docs/mod-development/sdk/commands/index.md`
- `docs/website/docs/mod-development/sdk/index.md`
- `docs/website/docs/mod-development/sdk/quick-start.md`
- `docs/website/docs/mod-development/sdk/mods/executeCommand.md`
- `docs/website/docs/mod-development/sdk/mods/index.md`
- `docs/website/docs/mod-development/menus.md`
- `docs/website/docs/mod-development/tray-menus.md`
- Japanese equivalents of all the above

### Project documentation

- Root `CLAUDE.md`
- `CLAUDE.md` (desktop_homunculus)

### SDK comments (~2 files)

- `packages/sdk/src/commands.ts` (JSDoc)
- `packages/sdk/src/mods.ts` (JSDoc)

### Rust comments (~3 files)

- `engine/crates/homunculus_utils/src/schema/mods.rs` (doc comments)
- `engine/crates/homunculus_http_server/src/route/mods.rs` (doc comments)
- `engine/crates/homunculus_mcp/src/handler/tools/system.rs` (doc comments)

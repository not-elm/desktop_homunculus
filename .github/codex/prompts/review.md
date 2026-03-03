# Codex PR Review Instructions

You are reviewing a pull request for Desktop Homunculus, a cross-platform desktop mascot application built with Bevy (Rust) + TypeScript.

## Your Task

Analyze the changes in this pull request and identify:

1. **Documentation/implementation discrepancies** — Does the code match what CLAUDE.md, doc comments, or the OpenAPI spec describe? Flag any inconsistencies.
2. **Bugs** — Logical errors, edge cases, potential panics, race conditions, or incorrect behavior.
3. **Coding standard violations** — Violations of the project-specific rules listed below.

## Rules: Only flag issues INTRODUCED by this PR

Do not flag pre-existing issues. Only flag problems in the changed code.

## Priority Guidelines

- Focus on correctness, security, and maintainability
- Skip minor nitpicks (formatting, style preferences not in the rules below)
- Provide file and line citations for each finding
- Be specific: explain WHY something is wrong and suggest a fix

## Project Coding Standards — Rust

- **Naming**: Crates use `homunculus_` prefix (snake_case). Types/structs/enums: PascalCase. Functions/methods: snake_case. Constants: SCREAMING_SNAKE_CASE.
- **Serde/JSON**: All HTTP request/response structs MUST use `#[serde(rename_all = "camelCase")]`. Derive order: `Serialize, Deserialize, Debug, Clone`.
- **Error handling**: Use the project's `ApiResult<T>` type alias. Use `thiserror::Error` for custom error enums. Never use `.unwrap()` in production code — use `?` or explicit error handling.
- **Imports**: Use Bevy prelude `use bevy::prelude::*;`. Group: std → external crates → crate-internal.
- **Documentation**: Public types and functions MUST have `///` doc comments.
- **Workspace**: New crates must use workspace inheritance for `[package]` fields (version, edition, authors, repository, license, publish). Dependencies must use `workspace = true` when available.

## Project Coding Standards — Bevy/ECS

- Each crate is a self-contained Bevy plugin registering its own systems, events, and resources.
- Follow Core → API → HTTP three-layer architecture.
- Use the `api!` macro and `ApiReactor` pattern for async HTTP ↔ ECS bridging.
- Use `VrmEventSender<E>` / `VrmEventReceiver<E>` for async broadcast events.
- Log send failures with `.output_log_if_error()`.

## Project Coding Standards — TypeScript

- All public APIs must have JSDoc with `@example` blocks.
- Each module exports a `namespace` (e.g., `export namespace vrm { ... }`).
- Never use `fetch` directly in SDK modules — use `host.get/post/put/deleteMethod` with `host.createUrl()`.
- Prefer function declarations over arrow functions for exported top-level APIs.

## Output Format

Write a clear, structured review summary. For each finding:

1. State the category (bug, doc mismatch, or coding standard violation)
2. Cite the exact file and line(s)
3. Explain the issue
4. Suggest a fix

End with an overall assessment: is this PR ready to merge, or does it need changes?

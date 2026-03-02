# Rust Coding Style

## Formatting & Linting

- Use `cargo fmt` (default rustfmt settings, no rustfmt.toml).
- Use `cargo clippy -- -D warnings`. The `type_complexity` lint is allowed workspace-wide.
- Run `make fix` to apply both: `cargo clippy --workspace --fix --allow-dirty && cargo fmt --all`.

## Naming

- Crates: `snake_case` prefixed with `homunculus_` (e.g., `homunculus_core`, `homunculus_api`).
- Types/structs/enums: `PascalCase` (e.g., `VrmApi`, `HttpState`).
- Functions/methods: `snake_case` (e.g., `list_expressions`, `set_expressions`).
- Constants: `SCREAMING_SNAKE_CASE`.

## Serde & JSON

- All HTTP request/response structs MUST use `#[serde(rename_all = "camelCase")]`.
- Derive order: `Serialize, Deserialize, Debug, Clone` (in that order).
- For Rust reserved keywords as field names, use `#[serde(rename = "loop")]`.
- Use default value helper functions: `fn default_volume() -> f64 { 1.0 }`.

## Error Handling

- Use the project's `ApiResult<T>` type alias (`Result<T, ApiError>`) for all API operations.
- Use `thiserror::Error` for custom error enums.
- Map errors to HTTP status codes via `impl IntoResponse for ApiError`.
- Use `.output_log_if_error()` for broadcast/channel send failures.
- Never use `.unwrap()` in production code; use `?` or explicit error handling.

## Imports

- Use Bevy prelude: `use bevy::prelude::*;`.
- Use crate-level imports: `use crate::error::ApiResult;`.
- Group imports: std → external crates → crate-internal.

## Function Granularity

- Extract functions at a granularity where the calling code reads naturally as prose.

## Documentation

- Public types and functions MUST have `///` doc comments.
- Include `# Usage` or `# Example` blocks in doc comments for complex APIs.
- Use `//!` for module-level documentation at the top of `lib.rs` / `mod.rs`.

## Edition & Workspace

- Rust edition 2024.
- Workspace version: `0.1.0-alpha.4`.
- License: MIT OR Apache-2.0.
- All new crates must be added to the workspace `Cargo.toml`.
- New crate `[package]` fields must use workspace inheritance:
  ```toml
  version.workspace = true
  edition.workspace = true
  authors.workspace = true
  repository.workspace = true
  license.workspace = true
  publish.workspace = true
  ```
- Dependencies must use `workspace = true` when available in `[workspace.dependencies]`. Add new shared dependencies to the workspace before referencing them.

# Rust Coding Style

## Formatting & Linting

- Use `cargo fmt` (default rustfmt settings, no rustfmt.toml).
- Do NOT use region-divider comments (e.g. `// ----------- Section Name -----------`). Use the file's natural structure (modules, impl blocks, doc comments) for organization instead.
- Use `cargo clippy -- -D warnings`. The `type_complexity` lint is allowed workspace-wide.
- Run `make fix` to apply both: `cargo clippy --workspace --fix --allow-dirty && cargo fmt --all`.
- Use the non-mod-rs file pattern (RFC 2126, Rust 1.30+) for module declarations. Place the module root in `foo.rs` alongside a `foo/` directory for submodules, instead of using `foo/mod.rs`.
  - ✅ `route/persona.rs` + `route/persona/create.rs`
  - ❌ `route/persona/mod.rs` + `route/persona/create.rs`

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

## Item Ordering

Arrange items top-down within a file. High-level components go at the top; lower-level details go toward the bottom.

### File-level ordering

1. Module declarations (`mod` / `pub mod`) and re-exports (`pub use`)
2. Imports (`use`)
3. Constants and type definitions (`const`, `static`, `struct`, `enum`, `type`)
4. Entry points (`main`, `Plugin` struct + `impl Plugin`, `api!` macro invocations)
5. Public functions and method implementations (`pub fn`, `pub async fn`)
6. Crate-internal functions (`pub(crate) fn`)
7. Private functions and helpers (`fn`)
8. Tests (`#[cfg(test)]`)

### Ordering within impl blocks

1. `pub` methods
2. `pub(crate)` methods
3. Private methods

### Principles

- **Callers above, callees below.** `main` → functions it calls → their sub-functions.
- **Place struct/enum definitions above the impl blocks that use them.** Group type definitions near the top of the file.
- **Bevy Plugins are entry points.** Place Plugin struct + `impl Plugin` above the system functions registered in `build()`.

### Exceptions

- `macro_rules!` macro definitions are exempt from ordering rules.

## Function Granularity

- Extract functions at a granularity where the calling code reads naturally as prose. The caller states "what" to do; the helper handles "how".
- Aim for function bodies under 20 lines. If a function exceeds this, look for a named sub-operation to extract.
- Inline closures passed to combinators (`map`, `and_then`, `match` arms, etc.) that exceed 3 lines should be extracted as named functions.

## Documentation

- Public types and functions MUST have `///` doc comments.
- Include `# Usage` or `# Example` blocks in doc comments for complex APIs.
- Use `//!` for module-level documentation at the top of `lib.rs` and module root files.

## Edition & Workspace

- Rust edition 2024.
- Workspace version: see `version.toml` at repo root.
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

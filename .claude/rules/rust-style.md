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

## Item Ordering

ファイル内のアイテムはトップダウンに配置する。上に高位の部品、下に行くほど低位の詳細。

### ファイル内の配置順序

1. モジュール宣言 (`mod` / `pub mod`)・再エクスポート (`pub use`)
2. インポート (`use`)
3. 定数・型定義 (`const`, `static`, `struct`, `enum`, `type`)
4. エントリポイント（`main`, `Plugin` struct + `impl Plugin`, `api!` マクロ呼び出し）
5. パブリック関数・メソッド実装 (`pub fn`, `pub async fn`)
6. クレート内部関数 (`pub(crate) fn`)
7. プライベート関数・ヘルパー (`fn`)
8. テスト (`#[cfg(test)]`)

### impl ブロック内の配置順序

1. `pub` メソッド
2. `pub(crate)` メソッド
3. プライベートメソッド

### 原則

- **呼び出す側が上、呼ばれる側が下。** `main` → それが呼ぶ関数 → さらにその下位関数。
- **struct/enum 定義は、それを使う impl より上に置く。** 型定義はファイル上部にまとめる。
- **Bevy Plugin はエントリポイント。** Plugin struct + `impl Plugin` を上に、`build()` から登録されるシステム関数を下に配置する。

### 例外

- `macro_rules!` によるマクロ定義は配置順序の規定対象外とする。

## Function Granularity

- Extract functions at a granularity where the calling code reads naturally as prose.
- 関数本体が自然言語のように読めるよう、意図を名前で表現したヘルパー関数に処理を切り出す。呼び出す側は「何をするか」を述べ、ヘルパーは「どうするか」を担当する。
- Aim for function bodies under 20 lines. If a function exceeds this, look for a named sub-operation to extract.
- Inline closures passed to combinators (`map`, `and_then`, `match` arms, etc.) that exceed 3 lines should be extracted as named functions.

## Documentation

- Public types and functions MUST have `///` doc comments.
- Include `# Usage` or `# Example` blocks in doc comments for complex APIs.
- Use `//!` for module-level documentation at the top of `lib.rs` / `mod.rs`.

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

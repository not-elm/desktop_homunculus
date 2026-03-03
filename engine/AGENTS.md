# Repository Guidelines

## Project Structure & Module Organization
- `src/main.rs`: desktop app entry point (Bevy plugin host).
- `crates/`: Rust workspace modules (core, API, HTTP server, VRM/WebView integrations).
- `ui/`: React + TypeScript frontends (settings/chat/menu) and shared components in `ui/core/`.
- `sdk/typescript/`: generated TypeScript client for the HTTP API.
- `assets/`: models, animations, mods, and configuration (including `.env`).
- `docs/` and `build/`: documentation assets and build outputs.

## Build, Test, and Development Commands
- `make dev` or `cargo run --features develop`: run the desktop app in dev mode.
- `npm run build`: build TypeScript UI bundles via Turbo.
- `cargo build`: build Rust binaries and crates.
- `make fix`: run formatting and lint fixes across the workspace.
- `cargo test --workspace`: run Rust tests.
- `npm run check-types` / `npm run lint`: TypeScript type-check and lint.

## Coding Style & Naming Conventions
- Rust: format with `cargo fmt`, lint with `cargo clippy -- -D warnings`.
- TypeScript: follow ESLint and type-checking (`npm run lint`, `npm run check-types`).
- Naming: use snake_case for Rust modules/functions, PascalCase for Rust types; camelCase for TS variables/functions and PascalCase for React components.

## Testing Guidelines
- Prefer unit tests for logic-heavy crates and integration tests for cross-crate behavior.
- Run `cargo test --workspace` for Rust changes; run `npm run build` or `npm run check-types` for UI/SDK changes.

## Commit & Pull Request Guidelines
- Follow conventional commits when possible (e.g., `feat: ...`, `fix: ...`, `docs: ...`).
- Existing history also uses short prefixes like `update:` or `add:`; keep messages concise and scoped.
- PRs should describe changes, list test commands run, and include screenshots/GIFs for UI changes.

## Configuration & API Notes
- Local config lives in `assets/.env` (e.g., `OPENAI_API_KEY=...`).
- If you change `crates/homunculus_http_server/src/**`, update `sdk/typescript/src/api/openapi.json`.

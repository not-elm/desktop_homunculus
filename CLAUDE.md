# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Layout

This is a monorepo for **Desktop Homunculus**, a cross-platform desktop mascot application built with the Bevy game engine. It renders transparent-window VRM 3D characters with WebView-based UI overlays.

```
desktop-homunculus/
├── engine/              # Main Bevy application (Rust workspace)
│   ├── crates/          # Rust plugin crates (homunculus_*)
│   │   └── homunculus_cli/  # Rust CLI binary (hmcs)
│   ├── src/main.rs      # App entry point — composes all plugins
│   └── assets/mods/     # Installed mods (runtime)
├── packages/
│   ├── sdk/             # @hmcs/sdk — TypeScript SDK for mods/extensions
│   ├── ui/              # @hmcs/ui — Shared React component library (Radix + Tailwind)
│   ├── cli/             # @hmcs/cli — Node CLI wrapper (distributes platform-specific Rust binary)
│   └── openclaw-plugin/ # @hmcs/openclaw-plugin — OpenClaw plugin bridging external agents
├── mods/                # Mods (NPM packages): persona/, character-settings/, settings/, menu/, assets/, elmer/, voicevox/, app-exit/, stt/, rpc-test/
├── docs/website/        # Docusaurus documentation site
└── sandbox/             # Dev sandbox — aggregates all mods for workspace linking validation
```

Sub-directories have their own CLAUDE.md with detailed architecture: `engine/`, `packages/sdk/`, `packages/ui/`.

## Development Commands

### Workspace (from repo root)

```bash
pnpm install          # Install all workspace dependencies
pnpm build            # Build all packages in dependency order (turbo)
pnpm dev              # Start all dev watchers (turbo)
pnpm check-types      # Type-check all packages (turbo)
pnpm test             # Run all TypeScript tests (turbo)
make setup            # pnpm install + engine tooling setup + CEF framework download
make debug            # pnpm build (excl. docs) + cargo run (debug with inspector)
make debug-cuda       # Same as debug but with CUDA STT support
make test             # pnpm test (TS) + cargo test --workspace (Rust)
make fix-lint         # cargo clippy --fix + cargo fmt (Rust) + pnpm lint:fix (TS)
make gen-open-api     # Regenerate OpenAPI spec + Docusaurus API docs + pnpm build
make release-macos    # pnpm build + native arch release → DMG
make release-windows  # pnpm build + MSI installer via WiX 4.x (Windows only)
make install-cli      # cargo install the hmcs CLI binary
make stage-runtime    # Download and stage Node.js, pnpm, tsx for bundling
make install-openclaw-plugin  # Build @hmcs/openclaw-plugin and install it into OpenClaw
```

### Engine (Rust) — run from `engine/`

```bash
make debug               # cargo run --features develop (bevy_egui inspector + CEF debug)
make test                # cargo test --workspace
make fix-lint            # cargo clippy --workspace --fix --allow-dirty && cargo fmt --all
make gen-open-api        # Regenerate OpenAPI spec via gen_openapi binary
```

Single test:
```bash
cargo test -p homunculus_http_server            # All tests in one crate
cargo test -p homunculus_http_server test_health # Single test by name
```

Release builds use `--profile dist` (not `--release`), which enables `lto = "thin"` and `strip = true`:
```bash
make release-macos           # Native arch → .app bundle → DMG
make release-macos-arm       # Apple Silicon
make release-macos-x86       # Intel
make release-macos-universal # Universal binary (ARM + x86)
```

### First-time setup (from `engine/`)

```bash
make setup               # Install all Rust/Node tools + download CEF framework (~300MB, skipped if present)
make setup-cef            # Download CEF framework only (macOS; skips if already installed)
```

### TypeScript SDK — run from `packages/sdk/`

```bash
pnpm build               # Rollup → ESM/CJS + bundled .d.ts
pnpm dev                 # Watch mode
pnpm check-types         # tsc --noEmit
```

### Shared UI Library — run from `packages/ui/`

```bash
pnpm build               # Vite library build → dist/ (ES + UMD + rolled .d.ts)
pnpm check-types         # tsc --noEmit
pnpm lint                # ESLint
```

### UI Mod Apps — run from `mods/{settings,menu}/ui/`

```bash
pnpm dev                 # Vite dev server
pnpm build               # Vite build → dist/
```

### Documentation Site — run from `docs/website/`

```bash
pnpm dev                 # Docusaurus dev server (English)
pnpm dev:ja              # Docusaurus dev server (Japanese)
pnpm build               # Production build
```

## Architecture Overview

The engine is built from ~20 independent Bevy plugins in `engine/crates/`, following a Core → API → HTTP layering. The HTTP API (Axum on `localhost:3100`) bridges async requests to Bevy's single-threaded ECS via the `ApiReactor` pattern. See `engine/CLAUDE.md` for detailed Rust architecture, code examples, and crate descriptions.

Asset path resolution: dev mode uses `assets/` relative to `CARGO_MANIFEST_DIR`; release uses `../Resources/assets` (inside `.app` bundle).

### OpenClaw Integration

DH does not include an in-process agent runtime. For AI-powered interaction, run
[OpenClaw](https://docs.openclaw.ai) externally and install the
[`@hmcs/openclaw-plugin`](packages/openclaw-plugin/) plugin. See
`docs/superpowers/specs/2026-04-18-openclaw-agent-integration-design.md` for the
full integration design.

### WebView Integration (bevy_cef)

UI components (settings, right-click menu) are React apps embedded via Chromium Embedded Framework (`bevy_cef`). They communicate with the Rust backend through the HTTP API and SSE-based pub/sub (`signals` module in the SDK). CEF runs with `disable-web-security` to allow cross-origin requests from WebViews to `localhost:3100`. A `CefFetchPlugin` proxies JavaScript `fetch` calls from WebViews through native `reqwest`. Transparent areas of a WebView do not capture mouse events — clicks pass through to the 3D scene beneath. This means CSS-only visibility changes (e.g., collapsing UI to a small icon) do not require resizing the WebView itself.

WebView keyboard shortcuts: `F1`/`F2` open/close DevTools, `Cmd+[`/`Cmd+]` navigate back/forward.

WebView sources can be URLs, inline HTML, or local mod assets using `{ "type": "local", "id": "mod-name:asset-id" }`.

### MOD System

Mods are pnpm workspace packages. Each mod's `package.json` must include a `"homunculus"` field declaring:
- **assets**: Objects with `path`, `type` (`vrm`, `vrma`, `sound`, `image`, `html`), and `description`. Asset IDs use format `"mod-name:asset-id"`.
- **menus** (optional): Right-click context menu entries that can open webviews.
- **tray** (optional): System tray menu entries (distinct from `menus`). Processed by `homunculus_tray` via `bevy_tray_icon`.

The `"homunculus.service"` script runs automatically as a long-running child process (service) at startup using `node --import tsx` (TypeScript files run directly without a build step; tsx is installed locally in the mods directory by `ensure_tsx()`). MOD commands are exposed via `"bin"` and invoked through the HTTP API (`POST /mods/{mod_name}/bin/{command}`). Mods use the `@hmcs/sdk` SDK.

**Mod discovery**: The engine runs `pnpm ls --parseable` in the mods directory (`~/.homunculus/mods/`) to discover installed mods, then reads each mod's `package.json` directly.

Source mods live in `mods/` (in the repo, for development). At runtime, mods are installed to `~/.homunculus/mods/` (configurable via `config.toml` `mods_dir` field). The built-in `@hmcs/assets` mod provides default VRMA animations (`vrma:idle-maid`, `vrma:grabbed`, `vrma:idle-sitting`) and sound effects (`se:open`, `se:close`).

### Frontend UI (Mod-Based)

UI apps live in `mods/` as mod packages — **settings** (`mods/settings/ui/`), **menu** (`mods/menu/ui/`), and **persona** (`mods/persona/ui/`). They are React 19 + Vite + Tailwind CSS v4 apps that import `@hmcs/ui` (from `packages/ui/`) as the shared component library. Build output goes to each mod's `ui/dist/` (bundled into a single `index.html` via `vite-plugin-singlefile` for CEF loading) and is declared as an asset in the mod's `package.json`.

**Design language**: Glassmorphism — semi-transparent backgrounds (`bg-primary/30`), `backdrop-blur-sm`, subtle borders (`border-white/20`), white text. This is the canonical style for all WebView UI overlays on the transparent Bevy window. The `@hmcs/ui` library is built on **shadcn/ui (new-york style)** with Radix UI primitives and **lucide-react** icons. Use the `cn()` utility from `@hmcs/ui` (clsx + tailwind-merge) for conditional class names.

### MCP Server (`engine/crates/homunculus_mcp/`)

Embedded Rust MCP server using Streamable HTTP transport, mounted at `/mcp` on the engine's Axum router (`localhost:3100/mcp`). Exposes static tools (character control, audio, webview), 5 resources (`homunculus://info`, `homunculus://characters`, `homunculus://mods`, `homunculus://assets`, `homunculus://rpc`), and 3 prompts. Uses the `rmcp` crate with `LocalSessionManager` for session isolation. MCP-specific fields and dynamic tool registration were removed from the RPC layer in favor of static exposure; RPC methods are invoked via HTTP (`POST /mods/{mod_name}/bin/{command}`), not auto-registered as MCP tools.

### Rust CLI (`engine/crates/homunculus_cli/`)

The `hmcs` binary is a Rust CLI built with `clap`. Current subcommands:
- `hmcs mod install|uninstall` — Install/uninstall mods to `~/.homunculus/mods/`
- `hmcs prefs list|get|set|delete` — Manage preferences in `~/.homunculus/preferences.db`
- `hmcs config` — Manage application configuration (`~/.homunculus/config.toml`)

## Important Workflows

- **After changing `engine/crates/homunculus_http_server/src/**`**: Update the OpenAPI spec. Run the `sync-api-docs` skill if available, or manually update `packages/sdk/src/` types to match.
- **After Rust changes**: Run `cargo test --workspace` from `engine/`.
- **After TypeScript SDK changes**: Run `pnpm build` from `packages/sdk/`.
- **After shared UI library changes**: Run `pnpm build` from `packages/ui/`, then rebuild consuming mod UIs.

## CI

- **Rust CI** (`ci-rust.yml`): fmt on `ubuntu-latest`, clippy + test on `{windows-latest, macos-latest}` matrix. Uses `--profile ci` and sccache. `--locked` flag means `Cargo.lock` must be kept committed and up to date.
  - `cargo fmt --all --check`
  - `cargo clippy --workspace --profile ci -- -Dwarnings`
  - `cargo test --workspace --locked --profile ci`
- **TypeScript CI** (`ci-ts.yml`): Runs on `ubuntu-latest`. `pnpm install --frozen-lockfile` → `pnpm build` → `pnpm check-types` → `pnpm test` → `pnpm lint`.

## Platform Notes

- **macOS**: Primary development platform. Default Bevy rendering backend.
- **Windows**: Supported. Known issue: black window background on Windows 11 with RTX GPUs.
- **Linux**: Planned, not yet supported.

## Requirements

- **Rust**: Latest stable toolchain
- **Node.js**: >= 22.0.0 (required by tsx for mod services)
- **pnpm**: 10.x (set via `packageManager` in root `package.json`)

## Key Dependencies

- **Bevy 0.18** — ECS game engine (Rust edition 2024)
- **bevy_cef** — Chromium Embedded Framework for WebViews (local path dependency at `../../bevys/bevy_cef`)
- **bevy_vrm1** — VRM/VRMA model loader (local path dependency at `../../bevys/bevy_vrm1`)
- **bevy_flurx** — Async task scheduling for Bevy (used by the ApiReactor pattern)
- **Axum** — HTTP server framework (for the REST API)

## Conventions

Coding style rules are defined in `.claude/rules/`:
- `rust-style.md` — Rust naming, formatting, serde (`camelCase` for all HTTP structs), error handling, imports, item ordering, workspace inheritance for new crates
- `bevy-patterns.md` — Plugin architecture, ApiReactor pattern, ECS patterns (prefer `try_insert` over `insert`)
- `ts-style.md` — TypeScript function granularity and extraction rules

Additional conventions:
- TypeScript SDK: All public APIs must have JSDoc with `@example` blocks. Each module exports a `namespace` (e.g., `export namespace vrm { ... }`). Never use `fetch` directly in SDK modules — always go through `host.ts` (`host.get/post/put/deleteMethod` with `host.createUrl()`). Prefer function declarations over arrow functions for exported top-level APIs.
- Commits: Conventional commits (`feat:`, `fix:`, `docs:`). Short prefixes like `update:`, `add:` also used.
- **Do NOT commit `docs/plans/`, `docs/superpowers/`, or `.superpowers/`**: These are local working files. Never include them in git commits.
- Application settings are stored in `~/.homunculus/config.toml` (TOML, snake_case keys: `port`, `mods_dir`).
- Logs are written to `~/.homunculus/Logs/log.txt` (daily rolling). Debug builds log at INFO level, release builds at ERROR.
- Preferences stored in SQLite at `~/.homunculus/preferences.db` (JSON key-value pairs).
- Workspace version: see `version.toml`. Bump with `make bump-version` (`scripts/bump_version.py`), verify with `make check-version`. Targets: `engine/Cargo.toml`, `packages/*/package.json`, `mods/*/package.json`.
- License: MIT/Apache-2.0 (Rust), MIT (TypeScript), CC-BY-4.0 (docs/assets).

## Build Profiles (Rust)

| Profile | `opt-level` | LTO | Strip | Usage |
|---------|-------------|-----|-------|-------|
| `dev` | 1 (deps: 3) | no | no | Local development (`make debug`) |
| `ci` | 1 (all) | no | no | CI lint + test |
| `release` | `"s"` | full | yes | Not typically used directly |
| `dist` | 2 | thin | yes | Distribution builds (`make release-*`) |

## Feature Flags (Rust)

- `develop` — Enables `bevy_egui` inspector + CEF debug mode
- `stt-cuda` — CUDA support for speech-to-text (`make debug-cuda`)
- `stt-metal` — Metal support for speech-to-text (macOS GPU acceleration)

## Workspace Lints

Clippy `type_complexity` and `result_large_err` are allowed workspace-wide.

# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

```bash
make debug               # cargo run --features develop (bevy_egui inspector + CEF debug)
make test                # cargo test --workspace
make fix-lint            # cargo clippy --workspace --fix --allow-dirty && cargo fmt --all
make build-open-api      # Rebuild OpenAPI HTML docs via redocly
make setup               # Install all Rust/Node tools + download CEF framework (~300MB, skipped if present)
make setup-cef            # Download CEF framework only (macOS; skips if already installed)
```

Single test:
```bash
cargo test -p homunculus_http_server            # All tests in one crate
cargo test -p homunculus_http_server test_health # Single test by name
```

## Architecture

This is the Rust workspace portion of the Desktop Homunculus monorepo — a transparent-window Bevy 0.18 application with VRM 3D characters and CEF-based WebViews.

### Three-Layer Architecture

The Rust crates form a layered architecture:

1. **Core** (`homunculus_core`) — Shared components, events, resources, and system parameters used across all plugins.
2. **API** (`homunculus_api`) — Domain-specific APIs (VRM, audio, webviews, effects, etc.) that bridge async code with Bevy's ECS via the `ApiReactor` pattern.
3. **HTTP** (`homunculus_http_server`) — Axum REST routes on `localhost:3100` that expose the API layer. Routes mirror API domain modules.

### ApiReactor Pattern (The Core Pattern)

This is the central pattern for bridging async HTTP handlers with Bevy's single-threaded ECS.

**Flow**: HTTP handler → `ApiReactor::schedule()` → `bevy_flurx` Reactor → one-shot Bevy system → result channel → HTTP response.

**Adding a new API endpoint** requires touching three layers:

1. **Define domain logic** in `homunculus_api`: Create an API struct using the `api!` macro, then implement methods that call `self.0.schedule()`:
   ```rust
   // The api! macro generates a newtype over ApiReactor with Clone, Resource, Deref, From
   api!(pub MyApi);

   impl MyApi {
       pub async fn do_something(&self, args: MyArgs) -> ApiResult<MyResult> {
           self.0.schedule(move |task| async move {
               // task.will(Update, once::run(system_fn).with(args)) runs a one-shot Bevy system
               task.will(Update, once::run(my_system).with(args)).await
           }).await
       }
   }
   ```

2. **Add route handler** in `homunculus_http_server/src/route/`:
   ```rust
   pub async fn my_handler(
       State(api): State<MyApi>,   // Extracted from HttpState via FromRef
       Json(body): Json<MyArgs>,
   ) -> HttpResult<MyResult> {
       api.do_something(body).await.into_http_result()
   }
   ```

3. **Register the route** in `homunculus_http_server/src/lib.rs` → `create_router()`, and add the API resource to `HttpState` (which implements `FromRef` for all domain APIs).

**Key types**: `ApiReactor` (sender), `TaskReceiver` (receiver), `ReactorTask` (provides `task.will()` for scheduling Bevy systems via `bevy_flurx`).

### Event Channel System

Events use async broadcast channels (`async-broadcast`, capacity 256 with overflow):
- `VrmEventSender<E>` / `VrmEventReceiver<E>` — typed broadcast resources
- `VrmEvent<E>` — wraps entity + payload
- Event types include: `OnClickEvent`, `OnDragStartEvent`, `OnDragEvent`, `OnDragEndEvent`, `OnPointerPressedEvent`, `VrmStateChangeEvent`, `ExpressionChangeEvent`, `VrmaPlayEvent`, `VrmaFinishEvent`, `PersonaChangeEvent`

### Core System Parameters

Custom `SystemParam` types available in `homunculus_core::system_param::prelude`:
- `Coordinate` — coordinate transformations between screen/world space
- `MascotTracker` — mascot position tracking
- `BoneOffsets` — bone position/offset calculations
- `Monitors` — monitor/display information
- `VrmAabb` — AABB calculations for VRM models
- `AssetResolver` — mod asset loading

### Plugin Composition

`src/main.rs` composes ~15 independent plugins. `HomunculusModPlugin` must be added first (enables asset loading). The `develop` feature flag adds `bevy_egui` inspector and CEF debug mode. `CefPlugin` runs with `disable-web-security` and includes a `CefFetchPlugin` that proxies JavaScript `fetch` calls from WebViews through native `reqwest`.

### Crates

- `homunculus_core` — Components (`Loading`, `AppWindow`, `VrmState`, `LinkedVrm`, `Persona`), events, resources, system parameters
- `homunculus_api` — `ApiReactor`, `api!` macro, domain APIs: `VrmApi`, `VrmAnimationApi`, `AudioSeApi`, `AudioBgmApi`, `PrefsApi`, `CameraApi`, `WebviewApi`, `EffectsApi`, `SpeechApi`, `SignalsApi`, `EntitiesApi`, `AssetsApi`, `ModsApi`, `ShadowPanelApi`, `AppApi`
- `homunculus_http_server` — Axum routes organized by domain in `src/route/`. `HttpState` holds all API resources. Test utilities: `test_app()`, `call()`, `assert_response()`. Includes command execution endpoint (`POST /commands/execute`) with NDJSON streaming output.
- `homunculus_mod` — MOD system: NPM package discovery, Node.js child processes
- `homunculus_speech` — Mora-based lip-sync for VRM models. Provides speech queue, vowel animation, and expression keyframe control. TTS audio is provided externally (e.g. via the Timeline API or MODs).
- `homunculus_prefs` — SQLite-backed preferences (`~/.homunculus/prefs.db`). Auto-persists VRM transforms. Key format: `"{asset_id}:transform"`, `"persona::{asset_id}"`
- `homunculus_utils` — Bevy-independent utilities shared across engine, CLI, and MCP: config loading (`~/.homunculus/config.toml`), path helpers (`homunculus_dir()`, `mod_dir()`), shared schema types, camera order constants
- Other plugins: `drag`, `effects`, `windows`, `hit_test`, `screen`, `sitting`, `shadow_panel`, `power_saver`, `audio`

### Command Execution

`POST /commands/execute` runs mod commands via `npx` and streams NDJSON events (`stdout`, `stderr`, `exit`). Request/response fields use camelCase (e.g. `timeoutMs`, `timedOut`). See `crates/homunculus_http_server/src/route/mods.rs`.

### Timeline API (Speech)

`SpeechApi::speak_with_timeline()` plays audio with synchronized expression keyframes, enabling TTS-agnostic lip-sync. VoiceVox TTS itself runs as an external MOD (`mods/voicevox/`), not in the engine. See `crates/homunculus_api/src/speech/timeline.rs`.

## Testing

The HTTP server crate has a test framework in `crates/homunculus_http_server/src/lib.rs`:
- `test_app()` creates a minimal Bevy app with `HomunculusApiPlugin` and returns `(App, Router)`
- `call()` / `call_any_status()` execute HTTP requests while pumping the Bevy game loop
- `assert_response()` deserializes JSON and asserts equality
- Tests sync async HTTP with Bevy's synchronous `app.update()` loop via polling

After changing Rust code, run `cargo test --workspace`.

## Important Workflows

- **After changing `crates/homunculus_http_server/src/**`**: Update the OpenAPI spec and SDK types in `../packages/sdk/src/`.
- HTTP server port is configured via `~/.homunculus/config.toml` (`port` field, defaults to `3100`). Config uses snake_case TOML keys (e.g., `mods_dir`).
- Logs are written to `~/.homunculus/Logs/log.txt` (daily rolling). Debug builds: INFO level. Release builds: ERROR level.
- WebView shortcuts: `F1`/`F2` open/close DevTools, `Cmd+[`/`Cmd+]` navigate back/forward.
- Asset path resolution: dev mode uses `assets/` relative to `CARGO_MANIFEST_DIR`; release uses `../Resources/assets` (inside `.app` bundle).

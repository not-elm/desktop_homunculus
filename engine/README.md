# Desktop Homunculus Engine

The Rust workspace powering Desktop Homunculus — a transparent-window [Bevy 0.18](https://bevyengine.org/) application that renders VRM 3D characters with [CEF](https://bitbucket.org/chromiumembedded/cef)-based WebView overlays.

## Architecture

The engine is organized as independent Bevy plugin crates in `crates/`, following a three-layer architecture:

1. **Core** (`homunculus_core`) — Shared components, events, resources, and system parameters
2. **API** (`homunculus_api`) — Domain-specific async APIs that bridge HTTP with Bevy's ECS via the ApiReactor pattern
3. **HTTP** (`homunculus_http_server`) — Axum REST routes on `localhost:3100` exposing the API layer

## Development

### First-time setup

```bash
make setup          # Install Rust/Node tools + download CEF framework (~300MB)
```

### Common commands

```bash
make debug          # cargo run --features develop (bevy_egui inspector + CEF debug)
make test           # cargo test --workspace
make fix-lint       # cargo clippy --fix + cargo fmt
```

### Single crate testing

```bash
cargo test -p homunculus_http_server            # All tests in one crate
cargo test -p homunculus_http_server test_health # Single test by name
```

## See Also

- [Root README](../README.md) — Project overview and downloads

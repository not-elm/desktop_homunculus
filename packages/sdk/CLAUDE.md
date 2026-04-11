# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TypeScript SDK (`@hmcs/sdk`) for building mods and extensions for Desktop Homunculus — a cross-platform desktop mascot application built with the Bevy game engine. The SDK communicates with the Bevy application via HTTP REST API on `localhost:3100`.

## Commands

```bash
pnpm install          # Install dependencies
pnpm build            # Production build (tsc --noEmit type-check + Rollup bundle)
pnpm dev              # Watch mode build
pnpm lint             # Biome (runs from repo root)
```

No test framework is configured.

## Build System

Rollup produces two outputs from `src/index.ts`:

1. **ESM + CJS modules** (`dist/*.js`, `dist/*.cjs`) — `preserveModules: true` keeps one output file per source file for tree-shaking
2. **Bundled type definitions** (`dist/index.d.ts`) — assembled from individual `dist/types/*.d.ts` via `rollup-plugin-dts`

## Architecture

All source is in `src/`. Each file is a TypeScript namespace that maps to a domain:

### Communication Layer
- **`host.ts`** — Internal HTTP client. All other modules use `host.get/post/put/deleteMethod` with `host.createUrl(path, params?)` to talk to the Bevy server at `localhost:3100`. Do not use `fetch` directly; go through `host`.
- **`signals.ts`** — Cross-process pub/sub via SSE (`EventSource`). `signals.stream<V>()` subscribes, `signals.send<V>()` publishes.

### Core Modules
- **`vrm.ts`** — VRM 3D character lifecycle: spawn, find, events (pointer-click, drag, state changes), voice synthesis (VoiceVox). Largest module.
- **`vrma.ts`** — VRMA animation management for VRM characters.
- **`gpt.ts`** — AI chat integration, model selection, system prompts, web search.
- **`entities.ts`** — Bevy ECS entity lookup by name, transform manipulation.
- **`webviews.ts`** — Embed HTML UIs in 3D space (global or attached to VRM bones).
- **`effects.ts`** — Sound effects and visual stamps.

### Utility Modules
- **`cameras.ts`** — Screen-to-world coordinate transforms.
- **`displays.ts`** — Multi-monitor detection and display info.
- **`preferences.ts`** — Persistent key-value storage (JSON serialized).
- **`settings.ts`** — Application configuration.
- **`mods.ts`** — Mod system integration, menu entries.
- **`math.ts`** — `Transform`, `Vec2`, `Vec3`, `Rect` types.
- **`shadowPanel.ts`**, **`scripts.ts`**, **`functions.ts`**, **`app.ts`** — Minor utilities.

### Key Patterns
- **Namespace-based API**: Each module exports a `namespace` (e.g., `export namespace vrm { ... }`). Consumers import namespaces directly.
- **Asset references**: String format `"mod-name:asset-name"` (e.g., `"elmer:idle"`, `"my-mod:click"`) for VRM models, sounds, webview HTML, images.
- **Options objects**: Configurable parameters use options/partial types (e.g., `SpawnVrmOptions`, `SpeakOnVoiceVoxOptions`).
- **Comprehensive JSDoc**: All public APIs have `@example` blocks and `@packageDocumentation` tags. Maintain this style when adding new APIs.

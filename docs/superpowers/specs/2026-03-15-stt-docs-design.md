# STT Documentation for docs/website

**Date:** 2026-03-15
**Scope:** Add STT (Speech-to-Text) SDK documentation and update overview pages to reflect the new feature added on the `tts` branch.

## Context

The `tts` branch adds a complete STT system: Rust microphone/VAD/Whisper pipeline, HTTP API (7 endpoints), and TypeScript SDK (`stt` namespace with session control, SSE streaming, and model management). The docs/website currently has no STT coverage.

## Changes

### New Files

#### `docs/website/docs/mod-development/sdk/stt/_category_.json`

Sidebar category configuration: label "STT", position 5.5. This places STT between audio (position 5) and speech (position 6) in the sidebar — grouping audio-related features together.

#### `docs/website/docs/mod-development/sdk/stt/index.md`

Overview and quick start page. Structure:

1. **Header** — One-line: real-time speech-to-text using local Whisper models
2. **Import** — `import { stt } from "@hmcs/sdk"`
3. **Quick Start** — Complete ~15-line example: download model → start session → stream results → cleanup
4. **Architecture** — Brief: microphone → VAD → Whisper inference, all local/offline, no cloud
5. **Model Sizes** — Table: tiny (32.5 MB), base (59.8 MB), small (189.8 MB, default), medium (491.8 MB) with speed/accuracy tradeoffs

6. **Prerequisites note** — Brief mention that STT requires microphone access; macOS will prompt for permission on first use. Error codes `no_microphone` and `microphone_permission_denied` are returned when access is unavailable.
7. **Sub-page links** — Session & Streaming, Models

Style follows VRM `index.md`: import, practical example first, conceptual background after. Target ~120 lines.

#### `docs/website/docs/mod-development/sdk/stt/session-and-streaming.md`

Session lifecycle and real-time streaming. Structure:

1. **Session Control** — `stt.session.start(options?)`, `.stop()`, `.status()`
   - `SttStartOptions` table: `language` (default `"auto"`), `modelSize` (default `"small"`)
   - State flow: Idle → Loading → Listening → Idle, with Error branching
   - Implicit restart: `start()` while Listening auto-stops and restarts. The returned state includes a `restarted: boolean` field indicating whether an existing session was stopped first.
   - Loading rejection: `start()` while Loading throws `session_loading` error
   - Note: `session_already_active` error code exists in the type system but is not reachable in practice because `start()` implicitly restarts. Document it in the error table with a note that it is reserved.
2. **Streaming** — `stt.stream(callbacks)` returns `SttStream`
   - Callbacks: `onResult(SttResult)`, `onStatus(SttState)`, `onSessionError(SttSessionError)`, `onStopped()`
   - Late-join sync: server sends current status immediately on connect
   - `SttStream.close()` for cleanup
   - Full example combining session start + streaming
3. **Error Handling** — `isSttError(e, code?)` type guard with example
   - Error code reference table: all `SttErrorCode` values with HTTP status and description
4. **Types** — Quick-reference tables:
   - `SttState` (tagged union: idle, loading, listening, error)
   - `SttResult` (text, timestamp, language)
   - `SttSessionError` (error, message)
   - `SttStartOptions` (language, modelSize)
   - `SttErrorCode` (all 10 values)
5. **Complete Workflow Example** — Service script streaming transcription

#### `docs/website/docs/mod-development/sdk/stt/models.md`

Model management. Structure:

1. **Listing** — `stt.models.list()` → `ModelInfo[]`
   - `ModelInfo` table: `modelSize`, `sizeBytes`, `path`
2. **Downloading** — `stt.models.download({ modelSize, signal? })` → `DownloadStream`
   - Brief explanation of `DownloadStream`'s dual-interface pattern: implements both `AsyncIterable<DownloadEvent>` and `PromiseLike<ModelDownloadResponse>`, enabling two usage modes from a single call
   - Await mode: `const result = await stt.models.download(...)` — final result only
   - Streaming mode: `for await (const event of stt.models.download(...))` — progress events
   - `DownloadEvent` table: `progress` (downloadedBytes, totalBytes, percentage), `complete`, `error`
   - `ModelDownloadResponse` table: `modelSize`, `status` (downloaded/alreadyExists/downloading), `path?`
   - AbortSignal cancellation example
3. **Model Size Reference** — Table with download sizes, speed/accuracy notes
4. **Example** — Download with progress percentage logging

Target ~80-100 lines.

### Modified Files

#### `docs/website/docs/mod-development/sdk/index.md`

- Add `stt` row to module map table: `| **stt** | import { stt } from "@hmcs/sdk" | Real-time speech-to-text transcription with local Whisper models. Session control, SSE streaming, and model management. |`
- Update module count in intro paragraph (currently says "18 modules", count actual rows in table after adding stt and use that number)

#### `docs/website/docs/getting-started/index.md`

Add bullet to **Key Features** list:
- **Speech-to-text (STT)** — Real-time voice transcription using local Whisper models. MODs can listen to microphone input and react to spoken words with no cloud dependency

## Source of Truth

All SDK documentation content is derived from:
- `packages/sdk/src/stt.ts` — TypeScript SDK implementation (types, methods, JSDoc)
- `engine/crates/homunculus_http_server/src/route/stt.rs` — HTTP endpoint definitions
- `engine/crates/homunculus_microphone/src/session.rs` — SttState, SttEvent types
- `engine/crates/homunculus_microphone/src/model.rs` — SttModelSize, download types

## Style Guidelines

- Follow existing SDK doc patterns (see `audio.md`, `signals.md` for single-page; `vrm/` for sub-page structure)
- Import block at top, practical code examples before conceptual explanations
- Quick-reference tables for types (Field | Type | Default | Description)
- Code examples use realistic MOD scenarios
- "Next Steps" links at bottom of each page. Cross-link to related modules: `speech.md` (output/lip-sync) and `audio.md` (sound effects) as complementary features

## Out of Scope

- OpenAPI `.api.mdx` regeneration (done separately)
- Japanese translations (English only)
- MCP tools docs (no MCP changes on this branch)

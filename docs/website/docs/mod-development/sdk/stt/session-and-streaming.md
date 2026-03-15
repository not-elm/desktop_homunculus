---
title: "Session & Streaming"
sidebar_position: 2
---

# Session & Streaming

Manage STT session lifecycle and receive real-time transcription results via Server-Sent Events (SSE).

## Session Control

Only one STT session can be active at a time.

### Start

`stt.session.start(options?)` begins transcription. Returns the new session state.

```typescript
// Start with defaults (auto language detection, small model)
const state = await stt.session.start();

// Start with specific options
const state = await stt.session.start({
  language: "ja",
  modelSize: "medium",
});
```

**Implicit restart:** If a session is already in `listening` state, calling `start()` will automatically stop it and start a new session. A console warning is logged when this happens.

**Loading rejection:** If a session is in `loading` state (model still loading), calling `start()` throws a `session_loading` error. Wait for loading to complete or call `stop()` first.

#### `SttStartOptions`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `language` | `string` | `"auto"` | Language code (ISO 639-1) or `"auto"` for detection |
| `modelSize` | `SttModelSize` | `"small"` | Whisper model size: `"tiny"`, `"base"`, `"small"`, `"medium"`, `"large-v3-turbo"`, or `"large-v3"` |

### Stop

`stt.session.stop()` ends the current session. Idempotent — safe to call even if no session is active.

```typescript
await stt.session.stop();
// Always returns { state: "idle" }
```

### Status

`stt.session.status()` returns the current session state without changing it.

```typescript
const status = await stt.session.status();
if (status.state === "listening") {
  console.log(`Listening in ${status.language} with ${status.modelSize} model`);
}
```

### State Flow

```
         start()          model loaded
  Idle ─────────► Loading ──────────► Listening
   ▲                │                    │
   │                │ error              │ stop() or
   │                ▼                    │ start() (restart)
   │              Error                  │
   │                │                    │
   └────────────────┴────────────────────┘
                  stop()
```

- **Idle** — No active session
- **Loading** — Model is being loaded into memory
- **Listening** — Actively capturing and transcribing audio
- **Error** — An error occurred; persists until `stop()` or `start()` clears it

## Streaming Results

`stt.stream(callbacks)` opens a persistent SSE connection and returns an `SttStream` instance. The server sends the current session state immediately on connect (late-join sync), so `onStatus` fires right away.

```typescript
const stream = stt.stream({
  onResult: (result) => {
    console.log(`[${result.language}] ${result.text}`);
  },
  onStatus: (state) => {
    console.log("State:", state.state);
  },
  onSessionError: (err) => {
    console.error(`Error: ${err.error} — ${err.message}`);
  },
  onStopped: () => {
    console.log("Session ended");
  },
});

// Close when done
stream.close();
```

All callbacks are optional — subscribe only to the events you need. Callbacks can be `async`; errors inside callbacks are caught and logged to the console.

### `StreamCallbacks`

| Callback | Argument | Description |
|----------|----------|-------------|
| `onResult` | `SttResult` | A transcription result was received |
| `onStatus` | `SttState` | Session state changed (also fires on connect) |
| `onSessionError` | `SttSessionError` | A session-level error occurred |
| `onStopped` | — | The session was stopped |

## Error Handling

Use `stt.isSttError(e, code?)` to check for STT-specific errors. It narrows the type to `HomunculusApiError`.

```typescript
try {
  await stt.session.start({ language: "ja" });
} catch (e) {
  if (stt.isSttError(e, "no_microphone")) {
    console.error("No microphone found");
  } else if (stt.isSttError(e, "model_not_available")) {
    // Download the model first
    await stt.models.download({ modelSize: "small" });
    await stt.session.start({ language: "ja" });
  } else if (stt.isSttError(e)) {
    console.error("STT error:", e.message);
  } else {
    throw e;
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `session_already_active` | 409 | Reserved. Not reachable in practice because `start()` implicitly restarts. |
| `session_loading` | 409 | Cannot start: model is still loading |
| `model_not_available` | 412 | Specified model has not been downloaded |
| `model_load_failed` | 500 | Model file exists but failed to load |
| `pipeline_failed` | 500 | Audio capture or inference pipeline failed |
| `no_microphone` | 503 | No microphone detected on this device |
| `microphone_permission_denied` | 503 | Microphone access was denied by the OS |
| `download_failed` | 500 | Model download failed (network error, etc.) |
| `invalid_model_size` | 422 | Unrecognized model size value |
| `invalid_language` | 422 | Unrecognized language code |

## Types

### `SttState`

Tagged union — use the `state` field to discriminate.

| Variant | Fields | Description |
|---------|--------|-------------|
| `{ state: "idle" }` | — | No active session |
| `{ state: "loading", language, modelSize }` | `language: string`, `modelSize: SttModelSize` | Model loading in progress |
| `{ state: "listening", language, modelSize }` | `language: string`, `modelSize: SttModelSize` | Actively transcribing |
| `{ state: "error", error, message }` | `error: string`, `message: string` | Session error occurred |

### `SttResult`

| Field | Type | Description |
|-------|------|-------------|
| `text` | `string` | Transcribed text |
| `timestamp` | `number` | Seconds since session start |
| `language` | `string` | Detected or specified language code |

### `SttSessionError`

| Field | Type | Description |
|-------|------|-------------|
| `error` | `string` | Error code |
| `message` | `string` | Human-readable error message |

## Complete Example

A MOD service that streams transcription and logs each result:

```typescript
import { stt, preferences } from "@hmcs/sdk";

// Ensure model is ready
const models = await stt.models.list();
if (!models.some((m) => m.modelSize === "small")) {
  await stt.models.download({ modelSize: "small" });
}

// Start session
await stt.session.start({ language: "auto", modelSize: "small" });

// Stream results and save to preferences
const stream = stt.stream({
  onResult: async (result) => {
    console.log(`[${result.timestamp.toFixed(1)}s] ${result.text}`);
    await preferences.save("stt::last-result", result);
  },
  onSessionError: (err) => {
    console.error(`STT error (${err.error}): ${err.message}`);
  },
  onStopped: () => {
    console.log("STT session ended");
  },
});

// Graceful shutdown
process.on("SIGTERM", async () => {
  await stt.session.stop();
  stream.close();
});
```

## Next Steps

- **[Models](./models)** -- Download and manage Whisper models
- **[STT Overview](./)** -- Quick start and architecture overview

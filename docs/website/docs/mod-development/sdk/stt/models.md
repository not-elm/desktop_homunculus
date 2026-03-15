---
title: "Models"
sidebar_position: 3
---

# Models

Download and manage Whisper models for speech-to-text. Models are stored locally and persist across sessions.

## Listing Models

`stt.models.list()` returns all downloaded and verified models.

```typescript
const models = await stt.models.list();
for (const m of models) {
  console.log(`${m.modelSize}: ${(m.sizeBytes / 1e6).toFixed(1)} MB at ${m.path}`);
}
```

### `ModelInfo`

| Field | Type | Description |
|-------|------|-------------|
| `modelSize` | `SttModelSize` | Model size (`"tiny"`, `"base"`, `"small"`, or `"medium"`) |
| `sizeBytes` | `number` | File size in bytes |
| `path` | `string` | Relative file path |

## Downloading Models

`stt.models.download(options)` returns a `DownloadStream` — an object that implements both `AsyncIterable<DownloadEvent>` and `PromiseLike<ModelDownloadResponse>`. This dual-interface pattern lets you choose between two usage modes from a single call:

### Await Mode (No Progress)

When awaited directly, the download completes and returns the final result. No progress events are emitted.

```typescript
const result = await stt.models.download({ modelSize: "small" });
console.log(result.status); // "downloaded" | "alreadyExists"
```

### Streaming Mode (With Progress)

When iterated with `for await...of`, yields progress events as the download proceeds.

```typescript
for await (const event of stt.models.download({ modelSize: "medium" })) {
  if (event.type === "progress") {
    console.log(`${event.percentage.toFixed(1)}%`);
  } else if (event.type === "complete") {
    console.log(`Downloaded to ${event.path}`);
  } else if (event.type === "error") {
    console.error(`Download failed: ${event.message}`);
  }
}
```

### Cancellation

Pass an `AbortSignal` to cancel a download in progress.

```typescript
const controller = new AbortController();

// Cancel after 30 seconds
setTimeout(() => controller.abort(), 30_000);

try {
  await stt.models.download({
    modelSize: "medium",
    signal: controller.signal,
  });
} catch (e) {
  if (stt.isSttError(e)) {
    console.log("Download cancelled or failed");
  }
}
```

### `ModelDownloadResponse`

Returned when awaiting `stt.models.download()`.

| Field | Type | Description |
|-------|------|-------------|
| `modelSize` | `SttModelSize` | The downloaded model size |
| `status` | `"downloaded" \| "alreadyExists" \| "downloading"` | Download outcome |
| `path` | `string \| undefined` | File path (present when `downloaded` or `alreadyExists`) |

### `DownloadEvent`

Yielded during streaming download.

| Variant | Fields | Description |
|---------|--------|-------------|
| `{ type: "progress" }` | `downloadedBytes: number`, `totalBytes: number`, `percentage: number` | Download progress update |
| `{ type: "complete" }` | `modelSize: SttModelSize`, `path: string` | Download completed successfully |
| `{ type: "error" }` | `message: string` | Download failed |

## Model Size Reference

| Size | Download Size | Use Case |
|------|--------------|----------|
| `"tiny"` | 32.5 MB | Quick prototyping, low-resource devices |
| `"base"` | 59.8 MB | Simple tasks, moderate accuracy |
| `"small"` | 189.8 MB | **Recommended.** Good accuracy for most languages |
| `"medium"` | 491.8 MB | Best accuracy, higher memory and CPU usage |

## Example: Download with Progress

```typescript
import { stt } from "@hmcs/sdk";

async function ensureModel(size: stt.SttModelSize) {
  const models = await stt.models.list();
  if (models.some((m) => m.modelSize === size)) {
    console.log(`Model ${size} already available`);
    return;
  }

  console.log(`Downloading ${size} model...`);
  for await (const event of stt.models.download({ modelSize: size })) {
    if (event.type === "progress") {
      const mb = (event.downloadedBytes / 1e6).toFixed(1);
      const total = (event.totalBytes / 1e6).toFixed(1);
      console.log(`  ${mb}/${total} MB (${event.percentage.toFixed(1)}%)`);
    } else if (event.type === "complete") {
      console.log(`  Done: ${event.path}`);
    }
  }
}

await ensureModel("small");
```

## Next Steps

- **[Session & Streaming](./session-and-streaming)** -- Start transcription and stream results
- **[STT Overview](./)** -- Quick start and architecture overview

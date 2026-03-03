---
title: "Settings"
sidebar_position: 16.3
---

# Settings

The `settings` module reads and updates application-wide rendering settings.

```typescript
import { settings } from "@hmcs/sdk";

// Read the current frame rate
const fps = await settings.fps();

// Set a new frame rate
await settings.setFps(30);
```

## API

### `settings.fps()`

Returns the current rendering frame rate.

- **Returns:** `Promise<number>` -- the current FPS value
- **HTTP:** `GET /settings/fps`

### `settings.setFps(fps)`

Updates the rendering frame rate.

- **Parameters:**
  - `fps` (`number`) -- the target frame rate (minimum 1)
- **Returns:** `Promise<void>`
- **HTTP:** `PUT /settings/fps` with body `{ "fps": <number> }`

## Example

```typescript
import { settings, shadowPanel } from "@hmcs/sdk";

// Low-power mode: reduce frame rate and dim overlay
await settings.setFps(15);
await shadowPanel.setAlpha(0.3);
```

---
title: "Direct HTTP"
sidebar_position: 16
---

# Direct HTTP

:::warning
Most developers should use the higher-level module APIs (`entities`, `Vrm`, `audio`, etc.) instead of calling the HTTP API directly. This module is for advanced use cases where no SDK wrapper exists yet.
:::

Low-level HTTP client for direct access to the Desktop Homunculus engine API at `localhost:3100`. The `host` namespace is used internally by every other SDK module.

## Import

```typescript
import { host } from "@hmcs/sdk";
```

## Configuration

By default the SDK connects to `http://localhost:3100`. Override the base URL if the engine runs on a different port:

```typescript
host.configure({ baseUrl: "http://localhost:4000" });
```

## URL Construction

`host.createUrl(path, params?)` builds a full API URL with optional query parameters.

```typescript
const url = host.createUrl("vrm");
// http://localhost:3100/vrm

const url = host.createUrl("entities", { name: "MyCharacter", root: 42 });
// http://localhost:3100/entities?name=MyCharacter&root=42
```

## Making Requests

All request methods automatically throw `HomunculusApiError` on non-OK responses.

```typescript
// GET
const response = await host.get(host.createUrl("vrm"));
const vrms = await response.json();

// POST with JSON body
await host.post(host.createUrl("vrm"), { asset: "my-mod:character" });

// PUT
await host.put(host.createUrl("vrm/123/state"), { state: "idle" });

// PATCH
await host.patch(host.createUrl("vrm/123/persona"), { profile: "cheerful" });

// DELETE
await host.deleteMethod(host.createUrl("vrm/123"));
```

## Streaming (NDJSON)

`host.postStream<T>(url, body?, signal?)` sends a POST request and returns an async generator that yields parsed NDJSON objects as they arrive.

```typescript
import { host, type HomunculusStreamError } from "@hmcs/sdk";

const stream = host.postStream<{ type: string; data: string }>(
  host.createUrl("commands/execute"),
  { command: "build" },
);

for await (const event of stream) {
  console.log(event);
}
```

## Error Handling

The SDK exports two error classes:

### HomunculusApiError

Thrown when the HTTP API returns a non-OK status (>= 400).

```typescript
import { HomunculusApiError } from "@hmcs/sdk";

try {
  await host.get(host.createUrl("vrm/999"));
} catch (err) {
  if (err instanceof HomunculusApiError) {
    console.error(err.statusCode); // 404
    console.error(err.endpoint);   // request URL
    console.error(err.body);       // response body text
  }
}
```

### HomunculusStreamError

Thrown when an NDJSON stream line cannot be parsed as JSON.

```typescript
import { HomunculusStreamError } from "@hmcs/sdk";

// err.rawLine contains the unparseable line
```

## Shadow Panel

The `shadowPanel` module controls the shadow overlay panel -- a fullscreen transparent layer used for atmospheric effects or focus dimming.

```typescript
import { shadowPanel } from "@hmcs/sdk";

// Dim the background
await shadowPanel.setAlpha(0.7);

// Read the current alpha
const current = await shadowPanel.alpha();

// Remove the overlay
await shadowPanel.setAlpha(0);
```

`alpha` ranges from `0` (fully transparent / invisible) to `1` (fully opaque).

## Next Steps

- **[SDK Overview](./)** -- Full module map and quick example

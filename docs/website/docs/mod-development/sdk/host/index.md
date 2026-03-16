---
sidebar_position: 1
---

# host

:::warning
Most developers should use the higher-level module APIs (`entities`, `Vrm`, `audio`, etc.) instead of calling the HTTP API directly. This module is for advanced use cases where no SDK wrapper exists yet.
:::

Low-level HTTP client for direct access to the Desktop Homunculus engine API at `localhost:3100`. The `host` namespace is used internally by every other SDK module.

## Import

```typescript
import { host, HomunculusApiError, HomunculusStreamError } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [configure](./configure) | Override the base URL for the engine API |
| [base](./base) | Returns the current base URL string |
| [baseUrl](./baseUrl) | Returns the current base URL as a `URL` object |
| [createUrl](./createUrl) | Build a full API URL with optional query parameters |
| [get](./get) | Perform a GET request |
| [post](./post) | Perform a POST request with JSON body |
| [put](./put) | Perform a PUT request with JSON body |
| [patch](./patch) | Perform a PATCH request with JSON body |
| [deleteMethod](./deleteMethod) | Perform a DELETE request |
| [postStream](./postStream) | POST and stream NDJSON response as an async generator |

---
sidebar_position: 100
---

# Type Definitions

### HomunculusApiError

Thrown when the HTTP API returns a non-OK status (>= 400).

```typescript
class HomunculusApiError extends Error {
  readonly statusCode: number;  // HTTP status code (e.g. 404, 500)
  readonly endpoint: string;    // The request endpoint URL
  readonly body: string;        // The response body text
}
```

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
class HomunculusStreamError extends Error {
  readonly rawLine: string;  // The raw line that failed to parse
}
```

```typescript
import { HomunculusStreamError } from "@hmcs/sdk";

// err.rawLine contains the unparseable line
```

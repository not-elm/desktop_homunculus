---
sidebar_position: 8
---

# put

Performs a PUT request with a JSON payload and automatic error handling. Throws [`HomunculusApiError`](./types#homunculusapierror) on non-OK responses.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the PUT request to |
| `body` | `B` (optional) | Request body that will be JSON-serialized |

## Returns

`Promise<Response>`

## Example

```typescript
await host.put(host.createUrl("vrm/123/state"), { state: "idle" });
```

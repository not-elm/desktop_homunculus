---
sidebar_position: 9
---

# patch

Performs a PATCH request with a JSON payload and automatic error handling. Throws `HomunculusApiError` on non-OK responses.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the PATCH request to |
| `body` | `B` (optional) | Request body that will be JSON-serialized |

## Returns

`Promise<Response>`

## Example

```typescript
await host.patch(host.createUrl("vrm/123/persona"), { profile: "cheerful" });
```

---
sidebar_position: 7
---

# post

Performs a POST request with a JSON payload and automatic error handling. Throws `HomunculusApiError` on non-OK responses.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the POST request to |
| `body` | `B` (optional) | Request body that will be JSON-serialized |

## Returns

`Promise<Response>`

## Example

```typescript
await host.post(host.createUrl("vrm"), { asset: "my-mod:character" });
```

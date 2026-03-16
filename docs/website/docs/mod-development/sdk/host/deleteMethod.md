---
sidebar_position: 10
---

# deleteMethod

Performs a DELETE request with automatic error handling. Throws `HomunculusApiError` on non-OK responses.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the DELETE request to |

## Returns

`Promise<Response>`

## Example

```typescript
await host.deleteMethod(host.createUrl("vrm/123"));
```

---
sidebar_position: 6
---

# get

Performs a GET request to the specified URL with automatic error handling. Throws `HomunculusApiError` on non-OK responses.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | `URL` | The URL to send the GET request to |

## Returns

`Promise<Response>`

## Example

```typescript
const response = await host.get(host.createUrl("vrm"));
const vrms = await response.json();
```

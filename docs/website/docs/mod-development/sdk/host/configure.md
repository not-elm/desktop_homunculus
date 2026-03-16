---
sidebar_position: 2
---

# configure

Override the SDK's base URL for the Desktop Homunculus HTTP server. By default the SDK connects to `http://localhost:3100`.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `options` | `{ baseUrl: string }` | Configuration options |

## Returns

`void`

## Example

```typescript
host.configure({ baseUrl: "http://localhost:4000" });
```

---
sidebar_position: 5
---

# createUrl

Builds a full API URL with optional query parameters.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `string` | The API endpoint path (relative to base URL) |
| `params` | `object` (optional) | Query parameters to append to the URL |

## Returns

`URL`

## Example

```typescript
const url = host.createUrl("vrm");
// http://localhost:3100/vrm

const url = host.createUrl("entities", { name: "MyCharacter", root: 42 });
// http://localhost:3100/entities?name=MyCharacter&root=42
```

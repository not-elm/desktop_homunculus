# Create URL

Creates a URL for the Desktop Homunculus API with optional query parameters.

## Parameters

- **path** (string): The API endpoint path (relative to base URL)
- **params** (object, optional): Query parameters to append to the URL

## Returns

A URL instance ready for use in HTTP requests, pointing to `http://localhost:3100/{path}`.

## Examples

### Simple Path

```typescript
const url = Deno.api.host.createUrl("vrm/all");
// Result: http://localhost:3100/vrm/all
```

### With Query Parameters

```typescript
const url = Deno.api.host.createUrl("entities", {
    name: "VRM",
    root: 123
});
// Result: http://localhost:3100/entities?name=VRM&root=123
```

## Related Functions

- **[get](./get.md)** - Uses created URLs for GET requests
- **[post](./post.md)** - Uses created URLs for POST requests
- **[put](./put.md)** - Uses created URLs for PUT requests
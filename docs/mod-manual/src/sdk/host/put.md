# PUT Request

Performs a PUT request with JSON payload and automatic error handling.

## Parameters

- **url** (URL): The URL to send the PUT request to (typically created with `createUrl`)
- **body** (any, optional): Request body that will be JSON-serialized

## Returns

A Promise that resolves to the Response object if successful.

## Throws

Will throw an error if the response status is not ok (status >= 400).

## Examples

### Basic PUT Request

```typescript
const url = Deno.api.host.createUrl("gpt/model");
const response = await Deno.api.host.put(url, {
    model: "gpt-4",
    vrm: 123
});
const result = await response.json();
```

## Related Functions

- **[createUrl](./createUrl.md)** - Creates URLs for PUT requests
- **[get](./get.md)** - GET requests for retrieving current state
- **[post](./post.md)** - POST requests for creating resources
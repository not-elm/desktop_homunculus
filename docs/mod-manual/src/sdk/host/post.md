# POST Request

Performs a POST request with JSON payload and automatic error handling.

## Parameters

- **url** (URL): The URL to send the POST request to (typically created with `createUrl`)
- **body** (any, optional): Request body that will be JSON-serialized

## Returns

A Promise that resolves to the Response object if successful.

## Throws

Will throw an error if the response status is not ok (status >= 400).

## Examples

### Basic POST Request

```typescript
const url = Deno.api.host.createUrl("gpt/chat");
const response = await Deno.api.host.post(url, {
    userMessage: "Hello!",
    options: {vrm: 123}
});
const chatResponse = await response.json();
```

### POST without Body

```typescript
// Empty body POST request
const url = Deno.api.host.createUrl("system/reset");
const response = await Deno.api.host.post(url);
const result = await response.json();
```

## Related Functions

- **[createUrl](./createUrl.md)** - Creates URLs for POST requests
- **[get](./get.md)** - GET requests for retrieving data
- **[put](./put.md)** - PUT requests for updates
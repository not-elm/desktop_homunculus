# GET Request

Performs a GET request to the specified URL with automatic error handling.

## Parameters

- **url** (URL): The URL to send the GET request to (typically created with `createUrl`)

## Returns

A Promise that resolves to the Response object if successful.

## Throws

Will throw an error if the response status is not ok (status >= 400).

## Examples

### Basic GET Request

```typescript
const url = Deno.api.host.createUrl("vrm/all");
const response = await Deno.api.host.get(url);
const vrms = await response.json();

console.log('Available VRMs:', vrms);
```

### GET with Query Parameters

```typescript
const url = Deno.api.host.createUrl("entities", {
    name: "MyCharacter"
});
const response = await Deno.api.host.get(url);
const entities = await response.json();
```

### Error Handling

```typescript
try {
    const url = Deno.api.host.createUrl("vrm/999"); // Non-existent VRM
    const response = await Deno.api.host.get(url);
    const data = await response.json();
} catch (error) {
    console.error('GET request failed:', error.message);
    // Error message includes URL, status, and response text
}
```

## Related Functions

- **[createUrl](./createUrl.md)** - Creates URLs for GET requests
- **[post](./post.md)** - POST requests with request body
- **[put](./put.md)** - PUT requests for updates
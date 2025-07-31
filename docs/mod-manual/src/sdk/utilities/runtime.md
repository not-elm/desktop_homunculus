# runtime

Detects the current JavaScript runtime environment.
This function helps determine whether the SDK is running in a browser, Node.js, or Deno environment, enabling
conditional logic and environment-specific optimizations.

## Parameters

None

## Returns

A string literal indicating the detected runtime environment:

- `"browser"` - Running in a web browser environment
- `"nodejs"` - Running in Node.js environment
- `"deno"` - Running in Deno environment

## Examples

### Basic Usage

```typescript
const env = Deno.api.functions.runtime();
console.log(`Running in ${env} environment`);

switch (env) {
    case 'browser':
        console.log('Browser environment detected');
        break;
    case 'nodejs':
        console.log('Node.js environment detected');
        break;
    case 'deno':
        console.log('Deno environment detected');
        break;
}
```

## Related Functions

- **[sleep](./sleep.md)** - Timing behavior may vary across runtimes
- [Preferences](../preferences/index.md) - Storage mechanisms differ by runtime
- [Commands](../commands/index.md) - Event handling varies by environment
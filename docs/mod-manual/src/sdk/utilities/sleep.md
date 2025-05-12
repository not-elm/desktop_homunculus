# sleep

Pauses execution for the specified number of milliseconds.
This utility function wraps setTimeout in a Promise, allowing for async/await syntax when you need to introduce delays
in your MOD code.

## Parameters

- **ms** (number): Number of milliseconds to pause execution

## Returns

A Promise that resolves after the specified time has elapsed.

## Examples

### Basic Usage

```typescript
// Simple delay
console.log('Starting...');
await Deno.api.functions.sleep(2000);
console.log('2 seconds later!');
```

## Related Functions

- **[runtime](./runtime.md)** - Environment detection for timing behavior
- [Commands](../commands/index.md) - Event timing and throttling
- [Effects](../effects/index.md) - Visual effect timing and synchronization
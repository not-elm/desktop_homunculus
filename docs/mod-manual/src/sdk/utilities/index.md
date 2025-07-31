# Utilities

The Utilities module provides helper functions for common operations and runtime detection throughout the Desktop
Homunculus SDK. These utility functions support timing operations, environment detection, and other frequently needed
functionality.

## Overview

The utilities module includes essential helper functions that make MOD development more convenient and efficient. These
utilities handle cross-platform concerns and provide consistent APIs for common operations.

### Key Features

- **Timing Control**: Sleep function for introducing delays and controlling execution flow
- **Runtime Detection**: Automatic detection of JavaScript execution environment
- **Cross-Platform Support**: Consistent behavior across browser, Node.js, and Deno environments
- **Development Helpers**: Utilities for debugging and development workflows

## Available Functions

### Core Utilities

- **[sleep](./sleep.md)** - Pauses execution for specified milliseconds
- **[runtime](./runtime.md)** - Detects current JavaScript runtime environment

## Basic Usage

### Delaying Execution

```typescript
// Simple delay
console.log('Starting...');
await Deno.api.functions.sleep(2000);
console.log('2 seconds later!');
```

## Related Documentation

- [Commands](../commands/index.md) - Event timing and debouncing
- [VRM](../vrm/index.md) - Animation sequencing and timing
- [Effects](../effects/index.md) - Visual effect timing and synchronization
# Host Communication

The Host API namespace provides low-level HTTP communication with the Desktop Homunculus server. This module handles the
foundational HTTP client functionality used internally by all other SDK modules.

## Overview

> [!Note]
> This module is primarily for internal SDK use.
>
> Most MOD developers should use the higher-level namespaces like `gpt`,
`vrm`, `commands`, etc., rather than calling host functions directly.

The host module provides:

- Base URL configuration for the Desktop Homunculus server
- URL construction utilities with query parameter support
- HTTP methods (GET, POST, PUT) with automatic error handling
- Type-safe response handling

## Base Configuration

The host module connects to the Desktop Homunculus server running on:

- **Base URL**: `http://localhost:3100`
- **Protocol**: HTTP with JSON content type
- **Error Handling**: Automatic status code validation

## Available Functions

### Core Functions

- **[createUrl](./createUrl.md)** - Creates URLs with optional query parameters
- **[get](./get.md)** - Performs GET requests with error handling
- **[post](./post.md)** - Performs POST requests with JSON payload
- **[put](./put.md)** - Performs PUT requests with JSON payload

## Basic Usage

While you typically won't use the host module directly, understanding its patterns can help with debugging and advanced
use cases:

```typescript
// Internal SDK usage example
const response = await Deno.api.host.get(
    Deno.api.host.createUrl("vrm/all")
);
const vrms = await response.json();

// URL construction with parameters
const url = Deno.api.host.createUrl("gpt/model", {vrm: 123});
// Results in: http://localhost:3100/gpt/model?vrm=123
```

## Error Handling

All host functions automatically validate HTTP response status codes and throw detailed errors for failed requests:

```typescript
try {
    const response = await Deno.api.host.get(url);
    const data = await response.json();
} catch (error) {
    // Error includes URL, status, and response text
    console.error('Request failed:', error.message);
}
```

## Internal Architecture

The host module serves as the foundation layer for all SDK communication:

1. **URL Construction**: Builds properly formatted API endpoints
2. **Request Headers**: Sets appropriate content types and headers
3. **Error Detection**: Validates response status codes
4. **Type Safety**: Ensures consistent response handling

## When to Use Direct Host Access

You might need direct host access in these scenarios:

- **Custom API Endpoints**: Accessing undocumented or experimental endpoints
- **Raw Response Data**: When you need the raw Response object
- **Advanced Error Handling**: Custom error processing requirements
- **Performance Optimization**: Bypassing higher-level abstractions

## Related Documentation

- [Commands](../commands/index.md) - Built on host for real-time communication
- [GPT Integration](../gpt/index.md) - Uses host for AI chat requests
- [VRM Management](../vrm/index.md) - Uses host for character operations
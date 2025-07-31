# webview.isClosed()

Checks whether the webview has been closed and is no longer available.

## Parameters

None.

## Returns

`Promise<boolean>` - Returns `true` if the webview is closed, `false` if it's still open

## Description

The `isClosed()` method allows you to check if a webview instance is still active. This is useful for conditional
operations, cleanup logic, and preventing operations on closed webviews.

## Example

```typescript
import {Webview} from '@homunculus/sdk';

const panel = await Webview.open({
    source: 'ui/status-panel.html'
});

// Check if webview is still open
if (!(await panel.isClosed())) {
    console.log('Panel is still open');
    // Safe to perform operations
} else {
    console.log('Panel has been closed');
    // Don't try to use the webview
}
```

## Related Documentation

- **[webview.close()](./close.md)** - Closing webviews
- **[Webview.open()](./open.md)** - Creating webviews
- **[Webview Management](./index.md)** - Overview of webview system
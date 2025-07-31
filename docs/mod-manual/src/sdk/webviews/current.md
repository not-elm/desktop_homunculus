# Webview.current()

Gets the current webview instance if called from within a webview context.

## Parameters

None.

## Returns

`Webview | undefined` - The current webview instance, or `undefined` if not in a webview context

## Description

The `current()` static method allows code running inside a webview to get a reference to its own webview instance. This
enables self-management operations like closing the webview from within, or checking its own status.

## Example

### Basic Self-Reference

```typescript
import {Webview} from '@homunculus/sdk';

// This code runs inside a webview's HTML/JavaScript
const currentWebview = Webview.current();

if (currentWebview) {
    console.log('Running inside webview:', currentWebview.entity);

    // The webview can close itself
    document.getElementById('closeButton')?.addEventListener('click', async () => {
        await currentWebview.close();
    });
} else {
    console.log('Not running in a webview context');
}
```

## Related Documentation

- **[Webview.open()](./open.md)** - Creating webviews
- **[webview.close()](./close.md)** - Closing webviews
- **[webview.isClosed()](./isClosed.md)** - Checking webview status
- **[Webview Management](./index.md)** - Overview of webview system
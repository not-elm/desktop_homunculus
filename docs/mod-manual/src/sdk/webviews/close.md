# webview.close()

Closes the webview window and removes it from the application.

## Parameters

None.

## Returns

`Promise<void>` - Resolves when the webview has been closed

## Description

The `close()` method removes the webview from the screen and cleans up its resources. If a close sound effect was
configured when the webview was opened, it will be played before the webview is destroyed.

## Example

```typescript
import {Webview} from '@homunculus/sdk';

// Open a webview
const panel = await Webview.open({
    source: 'ui/settings.html',
});

// Use the webview...

// Close it when done
await panel.close();
console.log('Webview closed');
```

## Related Documentation

- **[Webview.open()](./open.md)** - Creating webviews
- **[webview.isClosed()](./isClosed.md)** - Checking webview status
- **[Webview Management](./index.md)** - Overview of webview system
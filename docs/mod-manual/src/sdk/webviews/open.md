# Webview.open()

Creates and opens a new webview displaying content from a mod asset or URL.

## Parameters

- `options` (OpenOptions, optional) - Configuration options for the webview

### OpenOptions

```typescript
interface OpenOptions {
    asset: string;                     // Required mod asset ID for webview content
    caller?: number;                   // VRM entity ID that owns this webview
    transparent?: boolean;             // Whether background is transparent
    showToolbar?: boolean;             // Whether to show browser toolbar
    shadow?: boolean;                  // Whether window casts shadow (macOS only)
    position?: OpenPosition;           // Positioning configuration
    resolution?: [number, number];     // Width and height in pixels
    sounds?: {
        open?: string;                // Sound played when opening webview
        close?: string;               // Sound played when closing webview
    }
}
```

### OpenPosition

Position can be either fixed screen coordinates or VRM-relative:

```typescript
type OpenPosition = [number, number] | OpenAroundVrm

interface OpenAroundVrm {
    vrm: number;                       // VRM entity ID to attach to
    offset?: [number, number];         // Pixel offset from attachment point
    bone?: string;                     // VRM bone name to attach to
    tracking?: boolean;                // Whether to follow VRM movement
}
```

## Returns

`Promise<Webview>` - A new webview instance that can be controlled and closed

## Description

The `open()` method creates a new embedded browser window that can display HTML content from mod assets or external
URLs. Webviews can be positioned at fixed screen coordinates or attached to VRM characters for dynamic interfaces.

## Examples

### Basic Webview

```typescript
import {Webview} from '@homunculus/sdk';

// Simple webview with default settings
const panel = await Webview.open({
    asset: 'my-ui-mod::settings.html'
});
```

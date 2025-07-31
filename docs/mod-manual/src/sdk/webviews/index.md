# Webview Management

The Webview Management system provides comprehensive control over embedded web windows in Desktop Homunculus. This
powerful API allows you to create, position, and manage HTML-based user interfaces that can be attached to VRM
characters or positioned anywhere in 3D space.

## Overview

Webviews are embedded browser windows that serve as the primary UI technology for Desktop Homunculus MODs. They offer:

- **3D Positioning**: Place interfaces anywhere in 3D space relative to characters or screen coordinates
- **Character Attachment**: Attach UIs to specific VRM bones that track movement
- **Transparency**: Create seamless overlays that blend with the desktop
- **Full HTML/CSS/JS**: Use modern web technologies for rich interfaces
- **Real-Time Communication**: Integrate with the TypeScript SDK and HTTP API

## Core Concepts

### Positioning Types

- **Fixed Screen Position**: Traditional window positioning at pixel coordinates
- **VRM-Relative**: Positioned relative to VRM character bones
- **Tracking**: Webviews that follow character movement in real-time

### Visual Configuration

- **Transparency**: Alpha blending with desktop background
- **Custom Resolution**: Precise width/height control
- **Toolbar Management**: Show/hide browser controls
- **Audio Feedback**: Sound effects for open/close operations

## API Reference

### Webview Class

The main class for managing webview instances.

#### Constructor
```typescript
new Webview(entity: number)
```

Creates a webview instance wrapper around an entity ID. Typically not called directly - use static methods like `Webview.open()` instead.

#### Instance Methods

- **[close()](./close.md)** - Close the webview window
- **[isClosed()](./isClosed.md)** - Check if webview has been closed

#### Static Methods

- **[open(options)](./open.md)** - Create and open a new webview
- **[current()](./current.md)** - Get current webview instance

## Basic Example

```typescript
import { Webview } from '@homunculus/sdk';

// Create a simple settings panel
const settingsPanel = await Webview.open({
    source: 'my-mod/settings.html',
    position: [100, 100],
    resolution: [400, 300],
    transparent: false,
    showToolbar: true
});

// Close when done
await settingsPanel.close();
```


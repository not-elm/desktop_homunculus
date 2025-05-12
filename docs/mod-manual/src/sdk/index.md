# TypeScript SDK Reference

The Desktop Homunculus TypeScript SDK provides comprehensive APIs for building sophisticated MODs that interact with 3D
VRM characters, AI systems, and immersive user interfaces. This reference covers all available functions, types, and
usage patterns.

## SDK Overview

The SDK is organized into focused namespaces, each handling specific aspects of the Desktop Homunculus ecosystem. All
APIs are designed to be type-safe, well-documented, and easy to use in both startup scripts and webview interfaces.

### Core Features

- **VRM Character Management** - Load, control, and animate 3D VRM avatars
- **AI Integration** - GPT-powered chat with customizable personalities and voice synthesis
- **3D World Interaction** - Entity Component System for managing objects in 3D space
- **UI and Webview System** - Embed HTML interfaces in 3D space with character attachment
- **Effects and Media** - Visual effects, sound playback, and multi-monitor support
- **Cross-Process Communication** - Real-time messaging between MOD components
- **Persistent Data Storage** - Type-safe preference storage and state management

## Getting Started

### Installation and Import

In startup scripts, the SDK is available via the global `Deno.api` object:

```javascript
// Access SDK functions in startup scripts
const vrms = await Deno.api.vrm.findAll();
await Deno.api.gpt.chat('Hello!', {vrm: vrms[0].entity});
```

In webviews, use direct HTTP API calls or the webview bridge (when available):

```javascript
// HTTP API approach (recommended)
const response = await fetch('http://localhost:3100/vrm');
const vrms = await response.json();

// SDK bridge approach (limited availability)
if (window.DESKTOP_HOMUNCULUS_API) {
    const vrms = await window.DESKTOP_HOMUNCULUS_API.vrm.findAll();
}
```

### Quick Start Example

```typescript
// Basic MOD interaction example
async function createInteractiveCharacter() {
    // Spawn a VRM character
    const character = await Deno.api.vrm.spawn('my-mod::characters/assistant.vrm', {
        transform: {
            translation: [100, 0, 0],
            rotation: [0, 0, 0, 1],
            scale: [1, 1, 1]
        }
    });

    // Setup AI personality
    await Deno.api.gpt.saveSystemPrompt(
        'You are a friendly assistant who loves to help users.',
        {vrm: character.entity}
    );

    // Create floating interface
    await Deno.api.webviews.open('my-mod::chat.html', {
        position: {
            vrm: character.entity,
            bone: 'head',
            offset: [0, 100],
            tracking: true
        },
        transparent: true,
        showToolbar: false
    });

    // Setup interaction events
    const events = character.events();
    events.on('pointer-click', async () => {
        await Deno.api.effects.sound('my-mod::sounds/greeting.mp3');
        await Deno.api.gpt.chat('Hello! How can I help you today?', {
            vrm: character.entity,
            speaker: 1
        });
    });
}
```

# Startup Scripts

Startup scripts are JavaScript or TypeScript files that execute automatically when Desktop Homunculus launches.
These scripts run in a built-in Deno runtime environment and have full access to the TypeScript SDK, making them perfect
for initialization, automation, and background tasks.

## Overview

Startup scripts enable you to:

- Initialize your MOD's state and configuration
- Set up background tasks and timers
- Spawn default VRM characters
- Register event handlers and callbacks
- Perform data migration or cleanup tasks
- Connect to external services and APIs

## Configuration

Add startup scripts to the `startupScripts` array in your `mod.json`:

> [!WARNING]
> The execution order of startup scripts is not guaranteed.

```json
{
  "startupScripts": [
    "my-mod/scripts/init.js",
    "my-mod/scripts/background-tasks.js",
    "my-mod/scripts/character-setup.js"
  ]
}
```

## Deno Runtime Environment

It runs in a built-in [Deno](https://deno.com/) runtime.
This runtime allows you to use all Deno features, including unstable APIs like [
`Deno.cron`](https://docs.deno.com/examples/cron/).

### SDK Access

The TypeScript SDK is available via `Deno.api`.
Please refer to the [SDK reference](../sdk/index.md) for all available functions.

```javascript
// Access VRM functions
const vrms = await Deno.api.vrm.findAll();

// Use preferences
await Deno.api.preferences.save('my-setting', 'value');

// Play effects
await Deno.api.effects.sound('my-mod/sounds/startup.mp3');
```

## Common Startup Script Patterns

### 1. VRM Spawning

Set up your MOD's initial state:

```javascript
(async () => {
    const elmer = await Deno.api.Vrm.spawn("elmer/Elmer.vrm");
    console.log(await elmer.entity);
    console.log(await elmer.name());
    console.log(await elmer.state());
    await Deno.api.gpt.chat('Hello, Elmer!', {
        vrm: elmer.entity,
    });
})();
```

### 2. Background Tasks

You can use `setInterval` or `setTimeout` for periodic tasks.
Also, you can use [`Deno.cron`](https://docs.deno.com/examples/cron/) for cron-style scheduling.

```javascript
Deno.cron("Log a message", "* * * * *", () => {
    console.log("This will print once a minute.");
});
```

### 3. Listen Commands that sent by other processes

You can listen to commands sent by other processes.
This allows integration with external applications.

For more details on commands, refer to the [commands documentation](../sdk/commands/index.md).

```javascript

Deno.api.commands.stream('command-name', async (data) => {
    console.log("Received command data:", data);
});
```

## Next Steps

- **[Webview UI Development](../webview-ui/index.md)** - Create user interfaces for your MOD
- **[TypeScript SDK Reference](../sdk/index.md)** - Explore all available API functions
- **[Best Practices](../best-practices/index.md)** - Learn optimization and error handling techniques
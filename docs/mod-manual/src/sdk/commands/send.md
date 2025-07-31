# Sending Commands

Sends a command payload to all subscribers listening to the specified command channel.

This broadcasts the payload to all active EventSource connections that are streaming the same command type. The
operation is asynchronous and will complete once the command has been distributed to all subscribers.

## Parameters

- `command`: The command channel name to broadcast to
- `payload`: The data to send to all subscribers (must be JSON-serializable)

## Examples

### Send Notification to All MOD Windows

```typescript
// Send a notification to all mod windows
await commands.send("notifications", {
    type: "info",
    title: "Update Available",
    message: "A new version of the character is available",
    timestamp: Date.now()
});
```

## Related Functions

- [`stream()`](./stream.md) - Listen for commands on a specific channel
- [`webviews.open()`](../webviews/open.md) - Create webviews that can receive commands
- [`preferences.save()`](../preferences/save.md) - Persist data that's shared via commands

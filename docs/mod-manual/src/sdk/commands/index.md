# Commands API

The Commands API provides a pub/sub mechanism for cross-process communication. It allows external processes to communicate with the Desktop Homunculus application and its mods through event streaming.

## Key Features

- **Real-time Event Streaming**: Uses Server-Sent Events (SSE) for real-time communication
- **Command Broadcasting**: Send commands to multiple subscribers
- **Type-safe Payload Handling**: Full TypeScript support for message payloads
- **Cross-Process Communication**: Enable communication between different parts of the system

## Functions

- [`send()`](./send.md) - Send commands to all listeners on a channel
- [`stream()`](./stream.md) - Listen for commands on a specific channel

## Quick Example

```typescript
// Listen for custom events from external processes
const eventSource = commands.stream<{action: string, data: any}>(
  "my-custom-command",
  (payload) => {
    console.log("Received command:", payload.action, payload.data);
  }
);

// Send commands to all listeners
await commands.send("my-custom-command", {
  action: "update",
  data: { message: "Hello from external app!" }
});

// Clean up when done
eventSource.close();
```

## Common Use Cases

- **Inter-MOD Communication**: Enable mods to communicate with each other
- **External Integration**: Allow external applications to control Desktop Homunculus
- **Real-time Updates**: Stream live data between different components
- **Event Coordination**: Synchronize actions across multiple systems

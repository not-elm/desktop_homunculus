# Streaming Commands

Creates a persistent connection to stream command events of a specific type.

This establishes a Server-Sent Events (SSE) connection that will receive all commands sent to the specified command
channel. The connection remains open until explicitly closed.

## Parameters

- `command`: The command channel name to subscribe to
- `f`: Callback function to handle received payloads

## Returns

EventSource instance for managing the connection

## Examples

### Listen for User Interaction Events

```typescript
// Listen for user interaction events
interface UserAction {
    type: 'click' | 'hover' | 'scroll';
    position: [number, number];
    timestamp: number;
}

const userEventStream = commands.stream<UserAction>(
    "user-interactions",
    async (action) => {
        console.log(`User ${action.type} at`, action.position);

        // Process the user action
        switch (action.type) {
            case 'click':
                await effects.stamp('click-effect/ripple.gif');
                break;
            case 'hover':
                await effects.sound('ui-sounds/hover.wav');
                break;
        }
    }
);

// Later, close the stream
userEventStream.close();
```

### Cross-MOD Communication

```typescript
// MOD B: Listen for data from MOD A
interface SharedData {
    userId: string;
    preferences: {
        theme: 'dark' | 'light';
        volume: number;
    };
}

const dataSync = commands.stream<SharedData>(
    "user-data-sync",
    async (userData) => {
        console.log(`Received user data for ${userData.userId}`);

        // Apply preferences to this MOD
        if (userData.preferences.theme === 'dark') {
            document.body.classList.add('dark-theme');
        }

        // Store locally for this MOD
        await preferences.save('synced-user-data', userData);
    }
);
```

### Notification System

```typescript
// Listen for system notifications
interface Notification {
    type: 'info' | 'warning' | 'error' | 'success';
    title: string;
    message: string;
    timestamp: number;
    duration?: number;
}

const notificationStream = commands.stream<Notification>(
    "notifications",
    async (notification) => {
        // Show notification in UI
        showNotificationUI(notification);

        // Play appropriate sound
        const soundMap = {
            info: 'ui-sounds/info.wav',
            warning: 'ui-sounds/warning.wav',
            error: 'ui-sounds/error.wav',
            success: 'ui-sounds/success.wav'
        };

        await effects.sound(soundMap[notification.type]);

        // Log important notifications
        if (notification.type === 'error') {
            console.error(`[${notification.title}] ${notification.message}`);
        }
    }
);
```

## Use Cases

- **Real-time Updates**: Receive live data from external sources
- **Inter-MOD Communication**: Listen for messages from other MODs
- **User Interface Events**: Handle UI interactions from other components
- **System Monitoring**: Monitor application state changes
- **Gaming Events**: Handle real-time game events and player actions
- **Notification Systems**: Receive and display system notifications

## Related Functions

- [`send()`](./send.md) - Send commands to a specific channel
- [`webviews.open()`](../webviews/open.md) - Create webviews that can stream commands
- [`preferences.load()`](../preferences/load.md) - Load data that's shared via commands

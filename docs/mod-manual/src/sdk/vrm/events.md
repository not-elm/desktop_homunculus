# Vrm.events()

Returns an EventSource for receiving real-time events related to this VRM character.
This method provides access to user interactions, state changes, and other character-specific events.

## Parameters

None.

## Returns

`VrmEventSource` - An event source object for listening to VRM character events

## Description

The `events()` method creates a Server-Sent Events (SSE) connection that streams real-time events from the VRM
character. This enables reactive programming patterns where your mod can respond immediately to user interactions and
character state changes.

## Available Events

The VrmEventSource provides access to the following event types:

### Pointer Events

- **`pointer-click`** - Mouse click on the character
- **`pointer-press`** - Mouse button pressed down
- **`pointer-release`** - Mouse button released
- **`pointer-over`** - Mouse cursor enters character area
- **`pointer-out`** - Mouse cursor leaves character area
- **`pointer-cancel`** - Pointer event cancelled

### Drag Events

- **`drag-start`** - User starts dragging the character
- **`drag`** - Character is being dragged (continuous)
- **`drag-end`** - User stops dragging the character

### State Events

- **`state-change`** - Character state has changed

## Event Data Structures

### VrmPointerEvent

```typescript
interface VrmPointerEvent {
    globalViewport: [number, number]; // Cursor position in global viewport
}
```

### VrmDragEvent

```typescript
interface VrmDragEvent extends VrmPointerEvent {
    delta: [number, number]; // Change in cursor position since last event
}
```

### VrmMouseEvent

```typescript
interface VrmMouseEvent extends VrmPointerEvent {
    button: "Primary" | "Secondary" | "Middle"; // Which mouse button
}
```

### VrmStateChangeEvent

```typescript
interface VrmStateChangeEvent {
    state: string; // The new state of the VRM
}
```

## Examples

### Basic Event Handling

```typescript
import {Vrm} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::interactive.vrm');
const events = character.events();

// Handle click events
events.on('pointer-click', (event) => {
    console.log('Character clicked at:', event.globalViewport);
    console.log('Button:', event.button);
});

// Handle state changes
events.on('state-change', (event) => {
    console.log('Character state changed to:', event.state);
});

// Clean up when done
// events.close();
```

## Common Use Cases

### Interactive UI Creation

Create responsive interfaces that react to character interactions in real-time.

### Animation Triggering

Use events to trigger appropriate animations based on user actions and character states.

### State Management

Monitor and respond to character state changes for complex behavior systems.

### Multi-Character Coordination

Coordinate behaviors between multiple characters based on individual character events.

### User Feedback Systems

Provide immediate visual and audio feedback for user interactions.

## Related Documentation

- **[VrmEventSource](../VrmEventSource/index.md)** - Event source class details
- **[VrmEventSource.on()](../VrmEventSource/on.md)** - Event listener registration
- **[VrmEventSource.close()](../VrmEventSource/close.md)** - Cleanup and resource management
- **[Vrm.setState()](setState.md)** - Character state management
- **[VRMA Animation System](../vrma/index.md)** - Animation control integration
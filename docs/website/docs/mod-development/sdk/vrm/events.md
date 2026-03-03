---
title: "Events"
sidebar_position: 6
---

# Events

Subscribe to real-time character events using Server-Sent Events (SSE). The event system lets MODs react to pointer interactions, state changes, animation events, drag operations, and persona updates.

## Import

```typescript
import { Vrm } from "@hmcs/sdk";
```

## Creating an Event Source

`vrm.events()` returns a `VrmEventSource` connected to the character's SSE stream.

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();
```

### Disposable Support

`VrmEventSource` implements the `Disposable` protocol. Use TypeScript's `using` declaration to automatically close the connection when the variable goes out of scope:

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // eventSource is automatically closed at the end of this block
}
```

Without `using`, remember to call `close()` manually:

```typescript
const eventSource = character.events();
// ... register listeners ...

// When done:
eventSource.close();
```

## Registering Listeners

Use `.on(event, callback)` to register event handlers. Callbacks can be synchronous or async.

```typescript
const eventSource = character.events();

eventSource.on("state-change", (e) => {
  console.log("New state:", e.state);
});

eventSource.on("pointer-click", async (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]})`);
  console.log(`Button: ${e.button}`);
});
```

## Event Types

### State Events

| Event | Payload | Description |
|---|---|---|
| `state-change` | `{ state: string }` | Character state changed (e.g., `"idle"`, `"drag"`, `"sitting"`) |
| `expression-change` | `{ state: string }` | Expression changed |

### Animation Events

| Event | Payload | Description |
|---|---|---|
| `vrma-play` | `{ state: string }` | VRMA animation started playing |
| `vrma-finish` | `{ state: string }` | VRMA animation finished |

### Pointer Events

| Event | Payload | Description |
|---|---|---|
| `pointer-click` | `{ globalViewport, button }` | Character was clicked |
| `pointer-press` | `{ globalViewport, button }` | Mouse button pressed on character |
| `pointer-release` | `{ globalViewport, button }` | Mouse button released on character |
| `pointer-over` | `{ globalViewport }` | Mouse entered character area |
| `pointer-out` | `{ globalViewport }` | Mouse left character area |
| `pointer-move` | `{ globalViewport }` | Mouse moved within character area |
| `pointer-cancel` | `{ globalViewport }` | Pointer interaction cancelled |

### Drag Events

| Event | Payload | Description |
|---|---|---|
| `drag-start` | `{ globalViewport }` | Drag started |
| `drag` | `{ globalViewport, delta }` | Dragging in progress. `delta` is the cursor movement since last event. |
| `drag-end` | `{ globalViewport }` | Drag ended |

### Persona Events

| Event | Payload | Description |
|---|---|---|
| `persona-change` | `{ persona }` | Persona data was updated (profile, OCEAN, metadata) |

## Event Payloads

The `globalViewport` field is a `[number, number]` tuple representing cursor position in global screen coordinates (multi-monitor origin at the leftmost screen edge).

Mouse events include a `button` field with values `"Primary"`, `"Secondary"`, or `"Middle"`.

Drag events include a `delta` field -- a `[number, number]` tuple with the cursor movement since the previous event.

## Example: State Machine

A common pattern is using events to drive animation and behavior based on character state. This is the pattern used by the built-in Elmer MOD:

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

const animOption = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

// Start with idle animation
await character.playVrma({ asset: "vrma:idle-maid", ...animOption });

character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await character.playVrma({ asset: "vrma:idle-maid", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
    await character.playVrma({
      asset: "vrma:grabbed",
      ...animOption,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await character.playVrma({ asset: "vrma:idle-sitting", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  }
});
```

## Example: Click Counter

```typescript
const character = await Vrm.findByName("MyAvatar");
let clickCount = 0;

const eventSource = character.events();

eventSource.on("pointer-click", (e) => {
  if (e.button === "Primary") {
    clickCount++;
    console.log(`Clicked ${clickCount} times`);
  }
});

eventSource.on("pointer-over", () => {
  console.log("Mouse hovering over character");
});

eventSource.on("pointer-out", () => {
  console.log("Mouse left character");
});
```

## Types

```typescript
class VrmEventSource implements Disposable {
  on<K extends keyof EventMap>(
    event: K,
    callback: (event: EventMap[K]) => void | Promise<void>,
  ): void;
  close(): void;
}

interface VrmPointerEvent {
  globalViewport: [number, number];
}

interface VrmDragEvent extends VrmPointerEvent {
  delta: [number, number];
}

interface VrmMouseEvent extends VrmPointerEvent {
  button: "Primary" | "Secondary" | "Middle";
}

interface VrmStateChangeEvent {
  state: string;
}

interface PersonaChangeEvent {
  persona: Persona;
}
```

## Next Steps

- **[Spawn & Find](./spawn-and-find)** -- Create and locate characters to attach events to.
- **[Persona](./persona)** -- Listen for `persona-change` events.
- **[VRM Overview](./)** -- Full API reference table.

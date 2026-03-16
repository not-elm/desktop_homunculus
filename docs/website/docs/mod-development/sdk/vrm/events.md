---
title: "events"
sidebar_position: 32
---

# events

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.events()` returns a `VrmEventSource` connected to the character's Server-Sent Events (SSE) stream.

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();
```

Use [`VrmEventSource.on`](./VrmEventSource-on) to register event listeners, and [`VrmEventSource.close`](./VrmEventSource-close) to close the connection.

`VrmEventSource` implements the `Disposable` protocol for use with TypeScript's `using` declaration:

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // eventSource is automatically closed at the end of this block
}
```

## Available Events

| Event               | Payload                      | Description                                         |
| ------------------- | ---------------------------- | --------------------------------------------------- |
| `state-change`      | `{ state: string }`          | Character state changed (idle, drag, sitting, etc.) |
| `expression-change` | `{ state: string }`          | Expression changed                                  |
| `vrma-play`         | `{ state: string }`          | VRMA animation started playing                      |
| `vrma-finish`       | `{ state: string }`          | VRMA animation finished                             |
| `pointer-click`     | `{ globalViewport, button }` | Character was clicked                               |
| `pointer-press`     | `{ globalViewport, button }` | Mouse button pressed on character                   |
| `pointer-release`   | `{ globalViewport, button }` | Mouse button released on character                  |
| `pointer-over`      | `{ globalViewport }`         | Mouse entered character area                        |
| `pointer-out`       | `{ globalViewport }`         | Mouse left character area                           |
| `pointer-move`      | `{ globalViewport }`         | Mouse moved within character area                   |
| `pointer-cancel`    | `{ globalViewport }`         | Pointer interaction cancelled                       |
| `drag-start`        | `{ globalViewport }`         | Drag started                                        |
| `drag`              | `{ globalViewport, delta }`  | Dragging in progress (includes cursor delta)        |
| `drag-end`          | `{ globalViewport }`         | Drag ended                                          |
| `persona-change`    | `{ persona }`                | Persona data was updated                            |

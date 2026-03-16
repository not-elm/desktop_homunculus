---
title: "VrmEventSource.close"
sidebar_position: 34
---

# VrmEventSource.close

```typescript
import { Vrm } from "@hmcs/sdk";
```

`eventSource.close()` closes the SSE connection and stops receiving events.

```typescript
const eventSource = character.events();
// ... register listeners ...

// When done:
eventSource.close();
```

`VrmEventSource` also implements `Disposable`, so you can use TypeScript's `using` declaration for automatic cleanup:

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // eventSource.close() is called automatically here
}
```

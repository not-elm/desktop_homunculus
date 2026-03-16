---
sidebar_position: 5
---

# streamCommand

Returns an async generator that yields events as the command runs. Use this for long-running processes where you want real-time output. The last event is always an `exit` event.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `request` | `ExecuteCommandRequest` | Command execution parameters |
| `signal` | `AbortSignal` (optional) | Signal for cancellation |

## Returns

`AsyncGenerator<CommandEvent>`

## Example

```typescript
for await (const event of mods.streamCommand({ command: "build" })) {
  switch (event.type) {
    case "stdout":
      console.log(event.data);
      break;
    case "stderr":
      console.error(event.data);
      break;
    case "exit":
      console.log("Done, exit code:", event.exitCode);
      break;
  }
}
```

Cancel with an `AbortSignal`:

```typescript
const controller = new AbortController();
for await (const event of mods.streamCommand({ command: "slow" }, controller.signal)) {
  // ...
}
```

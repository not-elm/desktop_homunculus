---
sidebar_position: 4
---

# executeCommand

Runs a MOD command and returns the collected result after the process exits. Stdout and stderr are joined into strings. This is a convenience wrapper around `streamCommand` that buffers all output.

## Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `request` | [`ExecuteCommandRequest`](./types#executecommandrequest) | Command execution parameters |
| `signal` | `AbortSignal` (optional) | Signal for cancellation |

## Returns

`Promise<`[`CommandResult`](./types#commandresult)`>`

## Example

```typescript
const result = await mods.executeCommand({ command: "greet" });

if (result.exitCode === 0) {
  console.log("Output:", result.stdout);
} else {
  console.error("Error:", result.stderr);
}
```

Pass arguments, stdin data, or a custom timeout:

```typescript
const result = await mods.executeCommand({
  command: "compile",
  args: ["--target", "es2020"],
  stdin: JSON.stringify({ input: "data" }),
  timeoutMs: 60000,
});
```

Cancel with an `AbortSignal`:

```typescript
const controller = new AbortController();
const result = await mods.executeCommand({ command: "slow" }, controller.signal);
```

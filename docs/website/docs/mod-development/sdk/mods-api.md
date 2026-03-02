---
title: "Mods API"
sidebar_position: 14
---

# Mods API

Discover installed MODs, execute bin commands with buffered or streaming output, and query registered context menu entries.

## Import

```typescript
import { mods } from "@hmcs/sdk";
```

## Listing MODs

`mods.list()` returns metadata for every MOD discovered at startup. `mods.get(modName)` retrieves a single MOD by name.

```typescript
const allMods = await mods.list();
console.log(`${allMods.length} mods installed`);

// Find mods that expose bin commands
const withCommands = allMods.filter((m) => m.bin_commands.length > 0);

// Get a specific mod
const elmer = await mods.get("elmer");
console.log("Elmer assets:", elmer.asset_ids);
```

## Executing Commands (Buffered)

`mods.executeCommand(request)` runs a bin command and returns the collected result after the process exits. Stdout and stderr are joined into strings.

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

## Executing Commands (Streaming)

`mods.streamCommand(request)` returns an async generator that yields events as the command runs. Use this for long-running processes where you want real-time output.

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

Both methods accept an optional `AbortSignal` as a second argument for cancellation:

```typescript
const controller = new AbortController();
const result = await mods.executeCommand({ command: "slow" }, controller.signal);
```

## Menu Metadata

`mods.menus()` returns all context menu entries registered across installed MODs. Each entry is declared in a MOD's `package.json` under the `homunculus.menus` field.

```typescript
const menuItems = await mods.menus();
for (const item of menuItems) {
  console.log(`${item.modName}: ${item.text} -> ${item.command}`);
}
```

## Types

### ModInfo

```typescript
interface ModInfo {
  name: string;
  version: string;
  description?: string;
  author?: string;
  license?: string;
  has_main: boolean;
  bin_commands: string[];
  asset_ids: string[];
}
```

### ExecuteCommandRequest

```typescript
interface ExecuteCommandRequest {
  /** Bin command name to execute. */
  command: string;
  /** Arguments passed to the script. Max 64 args, each max 4096 chars. */
  args?: string[];
  /** Data written to process stdin. Stdin is closed after writing. Max 1 MiB. */
  stdin?: string;
  /** Timeout in milliseconds (1--300000). Defaults to 30000 (30s). */
  timeoutMs?: number;
}
```

### CommandEvent

The streaming API yields a union of three event types:

```typescript
type CommandEvent = CommandStdoutEvent | CommandStderrEvent | CommandExitEvent;

interface CommandStdoutEvent { type: "stdout"; data: string; }
interface CommandStderrEvent { type: "stderr"; data: string; }
interface CommandExitEvent {
  type: "exit";
  exitCode: number | null;
  timedOut: boolean;
  signal?: string;
}
```

### CommandResult

The buffered API returns a single result object:

```typescript
interface CommandResult {
  exitCode: number | null;
  timedOut: boolean;
  signal?: string;
  stdout: string;
  stderr: string;
}
```

### ModMenuMetadata

```typescript
interface ModMenuMetadata {
  id: string;
  modName: string;
  text: string;
  command: string;
}
```

## Next Steps

- **[Assets API](./assets-api)** -- Query the asset registry by type and MOD
- **[SDK Overview](./)** -- Full module map and quick example

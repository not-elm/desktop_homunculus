---
sidebar_position: 100
---

# Type Definitions

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
```

### CommandStdoutEvent

```typescript
interface CommandStdoutEvent { type: "stdout"; data: string; }
```

### CommandStderrEvent

```typescript
interface CommandStderrEvent { type: "stderr"; data: string; }
```

### CommandExitEvent

```typescript
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

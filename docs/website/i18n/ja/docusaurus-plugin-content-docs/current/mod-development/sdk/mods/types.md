---
sidebar_position: 100
---

# 型定義

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
  /** 実行する bin コマンド名。 */
  command: string;
  /** スクリプトに渡される引数。最大 64 個、各最大 4096 文字。 */
  args?: string[];
  /** プロセスの stdin に書き込まれるデータ。書き込み後に stdin は閉じられます。最大 1 MiB。 */
  stdin?: string;
  /** ミリ秒単位のタイムアウト（1--300000）。デフォルトは 30000（30 秒）。 */
  timeoutMs?: number;
}
```

### CommandEvent

ストリーミング API は 3 種類のイベント型の共用体を yield します：

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

バッファリング API は単一の結果オブジェクトを返します：

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

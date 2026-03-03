---
title: "mods"
sidebar_position: 14
---

# mods

インストール済み MOD の検出、バッファリングまたはストリーミング出力による bin コマンドの実行、登録済みコンテキストメニューエントリの照会を行います。

## インポート

```typescript
import { mods } from "@hmcs/sdk";
```

:::warning フィールド名の規則
`mods.get()` は **snake_case** フィールド名（`has_main`、`bin_commands`、`asset_ids`）の `ModInfo` を返します。これはアプリケーション情報エンドポイントが camelCase フィールド名を返すのとは異なります。フィールド名は将来のリリースで統一される予定です。
:::

## MOD の一覧取得

`mods.list()` は起動時に検出されたすべての MOD のメタデータを返します。`mods.get(modName)` は名前で単一の MOD を取得します。

```typescript
const allMods = await mods.list();
console.log(`${allMods.length} 個の MOD がインストールされています`);

// bin コマンドを公開している MOD を検索
const withCommands = allMods.filter((m) => m.bin_commands.length > 0);

// 特定の MOD を取得
const elmer = await mods.get("elmer");
console.log("Elmer のアセット:", elmer.asset_ids);
```

## コマンドの実行（バッファリング）

`mods.executeCommand(request)` は bin コマンドを実行し、プロセス終了後に収集された結果を返します。stdout と stderr は文字列に結合されます。

```typescript
const result = await mods.executeCommand({ command: "greet" });

if (result.exitCode === 0) {
  console.log("出力:", result.stdout);
} else {
  console.error("エラー:", result.stderr);
}
```

引数、stdin データ、カスタムタイムアウトを渡すこともできます：

```typescript
const result = await mods.executeCommand({
  command: "compile",
  args: ["--target", "es2020"],
  stdin: JSON.stringify({ input: "data" }),
  timeoutMs: 60000,
});
```

## コマンドの実行（ストリーミング）

`mods.streamCommand(request)` はコマンド実行中にイベントを yield する非同期ジェネレータを返します。リアルタイム出力が必要な長時間実行プロセスに使用してください。

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
      console.log("完了、終了コード:", event.exitCode);
      break;
  }
}
```

両方のメソッドは、キャンセル用にオプションの `AbortSignal` を第 2 引数として受け付けます：

```typescript
const controller = new AbortController();
const result = await mods.executeCommand({ command: "slow" }, controller.signal);
```

## メニューメタデータ

`mods.menus()` はインストール済み MOD 全体で登録されたすべてのコンテキストメニューエントリを返します。各エントリは MOD の `package.json` の `homunculus.menus` フィールドで宣言されます。

```typescript
const menuItems = await mods.menus();
for (const item of menuItems) {
  console.log(`${item.modName}: ${item.text} -> ${item.command}`);
}
```

## 型

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

## 次のステップ

- **[Assets API](./assets-api)** -- タイプと MOD によるアセットレジストリの照会
- **[SDK 概要](./)** -- 完全なモジュールマップとクイック例

---
title: "Commands"
sidebar_position: 18
---

# Commands

bin スクリプト用の stdin/stdout ユーティリティです。`@hmcs/sdk/commands` は、MOD の `package.json` の `"bin"` で宣言されるオンデマンドコマンド用の構造化された入力パースと出力ヘルパーを提供する **別エントリーポイント** です。

:::warning
MOD のメインスクリプトやブラウザ側のコードから `@hmcs/sdk/commands` をインポート **しないでください**。`process.stdin` やその他の Node.js API を使用しており、bin スクリプトのコンテキストでのみ利用可能です。
:::

## インポート

```typescript
import { input, output, StdinParseError } from "@hmcs/sdk/commands";
```

## 入力の読み取り

bin コマンドは `POST /mods/{mod_name}/bin/{command}` 経由で呼び出される際、エンジンから stdin で JSON を受け取ります。

### `input.parse(schema)`

stdin から JSON を読み取り、Zod スキーマでバリデーションします。バリデーション済みの型付きオブジェクトを返します。

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({
    entity: z.number(),
    text: z.union([z.string(), z.array(z.string())]),
    speaker: z.number().default(0),
  }),
);
```

内部的に 3 つのステップを実行します：
1. `input.read()` で stdin 全体を読み取り
2. 生の文字列を JSON としてパース
3. パースされたオブジェクトを提供された Zod スキーマでバリデーション

いずれかのステップが失敗すると [`StdinParseError`](#stdinparseerror) がスローされます。

### `input.parseMenu()`

メニューコマンドの stdin をパースし、リンクされたキャラクターの `Vrm` インスタンスを返します。メニューコマンドはメニュー UI から `{ "linkedVrm": <entityId> }` を stdin で受け取ります。

```typescript
import { input } from "@hmcs/sdk/commands";

const vrm = await input.parseMenu();
await vrm.setExpressions({ happy: 1.0 });
```

### `input.read()`

stdin 全体を生の UTF-8 文字列として読み取ります。JSON パースやバリデーションなしに生の文字列が必要な場合に便利です。

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
console.log("受信:", raw);
```

## 出力の書き込み

### `output.succeed(data)`

JSON 結果を stdout に書き込み、終了コード 0 でプロセスを終了します。成功した bin コマンドの最後の呼び出しとして使用してください。

```typescript
import { output } from "@hmcs/sdk/commands";

output.succeed({ greeting: `Hello, ${data.name}!` });
// stdout: {"greeting":"Hello, World!"}\n
// プロセスは終了コード 0 で終了
```

### `output.fail(code, message, exitCode?)`

構造化エラーを stderr に書き込み、プロセスを終了します。デフォルトの終了コードは `1` です。

```typescript
import { output } from "@hmcs/sdk/commands";

output.fail("NOT_FOUND", "エンティティが存在しません");
// stderr: {"code":"NOT_FOUND","message":"エンティティが存在しません"}\n
// プロセスは終了コード 1 で終了
```

### `output.write(data)`

JSON 結果を stdout に書き込みます。プロセスは **終了しません**。部分的な結果のストリーミングに便利です。

```typescript
output.write({ partial: "data" });
// stdout: {"partial":"data"}\n
```

### `output.writeError(code, message)`

構造化エラーを stderr に書き込みます。プロセスは **終了しません**。致命的でない警告に便利です。

```typescript
output.writeError("WARNING", "致命的でない問題");
// stderr: {"code":"WARNING","message":"致命的でない問題"}\n
```

## エラー処理

### `StdinParseError`

`input.parse()` が、stdin が空の場合、無効な JSON を含む場合、または Zod スキーマのバリデーションに失敗した場合にスローされるエラーです。`code` フィールドで失敗段階を識別できます：

| コード | 意味 |
|------|---------|
| `EMPTY_STDIN` | stdin に入力がありません |
| `INVALID_JSON` | stdin の内容が有効な JSON ではありません |
| `VALIDATION_ERROR` | JSON が Zod スキーマに一致しません（`details` フィールドに `ZodError` インスタンスが含まれます） |

```typescript
import { input, output, StdinParseError } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(schema);
  output.succeed(await processData(data));
} catch (err) {
  if (err instanceof StdinParseError) {
    output.fail(err.code, err.message);
  }
  throw err;
}
```

## 完全な例

バリデーション済みの入力を読み取り、構造化された出力を書き込む完全な bin コマンドスクリプトです：

```typescript
#!/usr/bin/env -S node --experimental-strip-types
import { z } from "zod";
import { input, output } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({ name: z.string() }),
);

output.succeed({ greeting: `Hello, ${data.name}!` });
```

## 次のステップ

- **[Bin コマンド](../bin-commands)** -- MOD の `package.json` で bin コマンドを宣言して呼び出す方法
- **[SDK クイックスタート](./quick-start)** -- インストールと最初のスクリプトのチュートリアル

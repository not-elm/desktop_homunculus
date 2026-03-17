---
title: "MOD コマンド"
sidebar_position: 7
---

# MOD コマンド

MOD コマンドは、MOD が `package.json` の `bin` フィールドを通じて公開するオンデマンドスクリプトです。起動時に自動実行されるサービスとは異なり、MOD コマンドは HTTP API 経由で明示的に呼び出された場合にのみ実行されます。

`package.json` での `bin` フィールドの宣言方法については[パッケージ設定](./project-setup/package-json.md#mod-commands)を参照してください。

## MOD コマンドスクリプトの作成

### シバン行

すべての TypeScript MOD コマンドは、コンパイルステップなしで直接実行を可能にするシバン行で始める必要があります：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />
```

シバンはシステムに対し、`tsx` を使ってファイルを実行するよう指示します。`tsx` はランタイム時に TypeScript をトランスパイルします。`/// <reference types="node" />` ディレクティブは Node.js の型定義（`process.stdin` など）を提供します。

:::warning
`tsx` には Node.js 22 以降が必要です。セットアップ手順は[インストール](/getting-started/installation)を参照してください。
:::

### `input.parse` による入力の解析

MOD コマンドは stdin 経由で JSON として入力を受け取ります。SDK は `@hmcs/sdk/commands` から `input.parse` を提供し、Zod スキーマを使って入力の読み取り、解析、バリデーションを行います。

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

const data = await input.parse(
  z.object({
    name: z.string(),
    count: z.number().default(1),
  })
);

console.log(data.name);  // バリデーション済みの文字列
console.log(data.count); // バリデーション済みの数値、デフォルトは 1
```

`input.parse` は 3 つのステップを実行します：

1. stdin 全体を UTF-8 文字列として**読み取り**
2. 文字列を JSON として**解析**
3. 解析されたオブジェクトを Zod スキーマに対して**バリデーション**

いずれかのステップが失敗すると、`StdinParseError` がスローされます（以下の[エラー処理](#エラー処理)を参照）。

:::note
`@hmcs/sdk/commands` は**別のエントリポイント**です。メインの `@hmcs/sdk` パッケージからは再エクスポートされていません。これは WebView のようなブラウザ環境では利用できない Node.js API（`process.stdin`）を使用するためです。
:::

JSON の解析やバリデーションなしで生の stdin 文字列が必要な場合は、代わりに `input.read` を使用してください：

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
```

### 出力の規約

MOD コマンドは stdout と stderr を通じて結果を伝達します：

| ストリーム | 用途 | フォーマット |
|--------|---------|--------|
| **stdout** | コマンド出力（結果、データ） | JSON 推奨 |
| **stderr** | エラーと診断メッセージ | フリーフォームテキスト |

SDK は構造化された出力用のヘルパー関数を提供します。`@hmcs/sdk/commands` からインポートしてください：

```typescript
import { output } from "@hmcs/sdk/commands";
```

**成功出力** — JSON 結果を stdout に書き込み、コード 0 で終了：

```typescript
output.succeed({ speakers: [...], count: 5 });
```

**エラー出力** — 構造化されたエラーを stderr に書き込み、非ゼロコードで終了：

```typescript
output.fail("NOT_FOUND", "Speaker 99 does not exist");
// コード 1（デフォルト）で終了

output.fail("TIMEOUT", "Request timed out", 2);
// コード 2 で終了
```

`output.fail` 関数は `code` と `message` フィールドを持つ JSON オブジェクトを stderr に書き込みます：
```json
{"code":"NOT_FOUND","message":"Speaker 99 does not exist"}
```

終了せずに出力を書き込みたい場合（例：中間の進捗状況）は、非終了バリアントを使用してください：

```typescript
import { output } from "@hmcs/sdk/commands";

output.write({ progress: 50 });         // stdout に書き込み、終了しない
output.writeError("WARN", "retrying");  // stderr に書き込み、終了しない
```

終了コードは標準的な規約に従います：

- **`0`** — 成功
- **非ゼロ** — 失敗（呼び出し側は `exit` イベントでこれを確認）

### エラー処理

`input.parse` が失敗すると、以下のいずれかのエラーコードを持つ `StdinParseError` がスローされます：

| コード | 原因 |
|------|-------|
| `EMPTY_STDIN` | 入力が受信されなかった（stdin が空またはホワイトスペースのみ） |
| `INVALID_JSON` | 入力が有効な JSON ではない |
| `VALIDATION_ERROR` | JSON が Zod スキーマに一致しない |

**パターン 1: 不正な入力で失敗** — 入力が必須の場合に使用。

```typescript
import { z } from "zod";
import { input } from "@hmcs/sdk/commands";

try {
  const data = await input.parse(
    z.object({ linkedVrm: z.number() })
  );
  // ... data を使用 ...
} catch (e) {
  console.error(e);
  process.exit(1);
}
```

:::tip
リンクされた VRM のみが必要なメニューコマンドの場合は、代わりに [`input.parseMenu()`](./menus.md#handling-menu-commands) を使用してください。スキーマの処理を行い、`Vrm` インスタンスを直接返します。
:::

**パターン 2: デフォルト値へのフォールバック** — 入力がオプションの場合に使用。

```typescript
import { z } from "zod";
import { input, StdinParseError } from "@hmcs/sdk/commands";

const defaults = { host: "http://localhost:50021" };
let parsed = defaults;
try {
  parsed = await input.parse(
    z.object({ host: z.string().default(defaults.host) })
  );
} catch (err) {
  if (!(err instanceof StdinParseError)) throw err;
  // stdin が空または不正な場合はデフォルトを使用
}
```

## 実行

### HTTP API

MOD コマンドは HTTP API 経由で呼び出します：

```
POST http://localhost:3100/commands/execute
Content-Type: application/json
```

### リクエストパラメータ

| フィールド | 型 | 必須 | 説明 |
|-------|------|----------|-------------|
| `command` | `string` | はい | 実行するコマンド名（`bin` で宣言されたもの） |
| `args` | `string[]` | いいえ | スクリプトに渡す引数。最大 64 項目、各最大 4096 文字。 |
| `stdin` | `string` | いいえ | プロセスの stdin に書き込まれるデータ。書き込み後に stdin はクローズされます。最大 1 MiB。 |
| `timeoutMs` | `number` | いいえ | タイムアウト（ミリ秒）。範囲：1〜300,000。デフォルト：30,000（30 秒）。 |

### レスポンスフォーマット

レスポンスは NDJSON（改行区切り JSON）ストリームです。各行は 3 つのイベントタイプのいずれかです：

**`stdout`** — スクリプトからの標準出力の 1 行：
```json
{"type":"stdout","data":"Hello, world!"}
```

**`stderr`** — 標準エラー出力の 1 行：
```json
{"type":"stderr","data":"Warning: using default config"}
```

**`exit`** — プロセスが終了した（常に最後のイベント）：
```json
{"type":"exit","code":0,"timedOut":false}
```

`exit` イベントにはプロセスが正常終了ではなくシグナルで終了した場合、`signal` フィールド（例：`"15"`）も含まれることがあります。

### SDK ラッパー

`@hmcs/sdk` は他の MOD スクリプトから MOD コマンドを呼び出すための 2 つの便利な関数を提供しています：

- **`mods.executeCommand(request)`** — すべての出力をバッファリングし、`stdout`、`stderr`、`exitCode` を持つ単一の `CommandResult` を返す
- **`mods.streamCommand(request)`** — リアルタイムストリーミング用の `AsyncGenerator<CommandEvent>` を返す

詳細は [Mods API](/reference/sdk/mods/) リファレンスを参照してください。

## 完全な例

入力パラメータに基づいて挨拶メッセージを構築する完全な MOD コマンドです。

**`package.json`**（関連フィールド）：

```json
{
  "name": "my-mod",
  "type": "module",
  "bin": {
    "my-mod:greet": "commands/greet.ts"
  },
  "dependencies": {
    "@hmcs/sdk": "latest",
    "zod": "^3.25.0"
  }
}
```

**`commands/greet.ts`**：

```typescript
#!/usr/bin/env tsx

/// <reference types="node" />

import { z } from "zod";
import { input, StdinParseError } from "@hmcs/sdk/commands";

// デフォルト付きの入力スキーマを定義
const schema = z.object({
  name: z.string().default("World"),
  language: z.enum(["en", "ja"]).default("en"),
});

// stdin を解析し、空の場合はデフォルトにフォールバック
const defaults = { name: "World", language: "en" as const };
let parsed: z.infer<typeof schema> = defaults;
try {
  parsed = await input.parse(schema);
} catch (err) {
  if (!(err instanceof StdinParseError)) throw err;
}

// 挨拶を構築
const greetings = { en: "Hello", ja: "こんにちは" };
const greeting = greetings[parsed.language];
const message = `${greeting}, ${parsed.name}!`;

// JSON として出力
console.log(JSON.stringify({ message }));
```

**`curl` での呼び出し：**

```bash
# 入力あり
curl -X POST http://localhost:3100/commands/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "my-mod:greet", "stdin": "{\"name\": \"Alice\", \"language\": \"ja\"}"}'

# 入力なし（デフォルトを使用）
curl -X POST http://localhost:3100/commands/execute \
  -H "Content-Type: application/json" \
  -d '{"command": "my-mod:greet"}'
```

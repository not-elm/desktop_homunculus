---
sidebar_position: 1
---

# commands

MOD コマンドスクリプト用の stdin/stdout ユーティリティです。`@hmcs/sdk/commands` は、MOD の `package.json` の `"bin"` で宣言される MOD コマンド用の構造化された入力パースと出力ヘルパーを提供する **別エントリーポイント** です。

:::warning
MOD のメインスクリプトやブラウザ側のコードから `@hmcs/sdk/commands` をインポート **しないでください**。`process.stdin` やその他の Node.js API を使用しており、MOD コマンドスクリプトのコンテキストでのみ利用可能です。
:::

## インポート

```typescript
import { input, output } from "@hmcs/sdk/commands";
```

## 関数

### 入力

| 関数 | 説明 |
|----------|-------------|
| [input.parse](./input-parse) | stdin から JSON を読み取り、Zod スキーマでバリデーション |
| [input.parseMenu](./input-parseMenu) | メニューコマンドの stdin をパースし、`Vrm` インスタンスを返す |
| [input.read](./input-read) | stdin 全体を生の UTF-8 文字列として読み取る |

### 出力

| 関数 | 説明 |
|----------|-------------|
| [output.succeed](./output-succeed) | JSON 結果を stdout に書き込み、終了コード 0 で終了 |
| [output.fail](./output-fail) | 構造化エラーを stderr に書き込み、プロセスを終了 |
| [output.write](./output-write) | JSON 結果を stdout に書き込む（プロセス終了なし） |
| [output.writeError](./output-writeError) | 構造化エラーを stderr に書き込む（プロセス終了なし） |

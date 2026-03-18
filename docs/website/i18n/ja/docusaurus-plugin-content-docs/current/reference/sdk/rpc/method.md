---
sidebar_position: 3
---

# method

型付きの RPC メソッド定義を作成します。オプションで Zod による入力バリデーションをサポートします。

## シグネチャ

**入力バリデーションあり：**

```typescript
rpc.method<I, O>(def: {
  description?: string;
  timeout?: number;
  input: ZodType<I>;
  handler: (params: I) => Promise<O>;
}): RpcMethodDef<I, O>
```

**入力なし：**

```typescript
rpc.method<O>(def: {
  description?: string;
  timeout?: number;
  handler: () => Promise<O>;
}): RpcMethodDef<unknown, O>
```

## パラメータ

| パラメータ | 型 | 必須 | 説明 |
|-----------|------|----------|-------------|
| `description` | `string` | いいえ | 人間が読める説明（RPC レジストリに公開されます） |
| `timeout` | `number` | いいえ | タイムアウト（ミリ秒単位、デフォルト: 30000） |
| `input` | `ZodType<I>` | いいえ | 入力バリデーション用の Zod スキーマ |
| `handler` | `(params: I) => Promise<O>` または `() => Promise<O>` | はい | リクエストを処理する非同期関数。入力なしのオーバーロードでは、ハンドラーは引数を取りません。 |

## 戻り値

[`RpcMethodDef<I, O>`](./types#rpcmethoddefi-o)

## 使用例

```typescript
import { rpc } from "@hmcs/sdk/rpc";
import { z } from "zod";

// 入力バリデーションあり
const speak = rpc.method({
  description: "Speak a message",
  timeout: 10000,
  input: z.object({ text: z.string() }),
  handler: async ({ text }) => {
    return { spoken: true };
  },
});

// 入力なし
const status = rpc.method({
  description: "Get current status",
  handler: async () => {
    return { ready: true };
  },
});
```

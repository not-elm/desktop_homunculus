---
sidebar_position: 2
---

# serve

エンジンが割り当てたポートで HTTP サーバーを起動し、メソッドをエンジンに登録し、グレースフルシャットダウンを処理します。

## シグネチャ

```typescript
rpc.serve(options: RpcServeOptions): Promise<RpcServer>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `options.methods` | `Record<string, `[`RpcMethodEntry`](/reference/sdk/rpc/types#rpcmethodentry)`>` | メソッド名から定義またはハンドラー関数へのマップ |

## 戻り値

[`RpcServer`](/reference/sdk/rpc/types#rpcserver)

## 動作

1. 環境変数 `HMCS_RPC_PORT`、`HMCS_MOD_NAME`、`HMCS_PORT` を読み取ります（MOD サービスの起動時にエンジンによって設定されます）。
2. `127.0.0.1:{HMCS_RPC_PORT}` で待ち受ける HTTP サーバーを作成します。
3. エクスポネンシャルバックオフで `POST /rpc/register` を呼び出し、メソッドをエンジンに登録します（最大 10 回リトライ、100 ms → 5 s）。
4. グレースフルシャットダウン用の `SIGTERM` ハンドラーをインストールします。

## エラーレスポンス

MOD の RPC サーバーにリクエストが到着した際、以下のレスポンスを返す場合があります：

| ステータス | 条件 |
|--------|-----------|
| 400 | 無効な JSON または Zod バリデーション失敗 |
| 404 | 不明なメソッド名 |
| 405 | POST 以外のリクエスト |
| 500 | ハンドラーがエラーをスロー |

## 使用例

```typescript
import { rpc } from "@hmcs/sdk/rpc";
import { z } from "zod";

const server = await rpc.serve({
  methods: {
    greet: rpc.method({
      input: z.object({ name: z.string() }),
      handler: async ({ name }) => ({ message: `Hello, ${name}!` }),
    }),
    ping: async () => ({ pong: true }),
  },
});
```

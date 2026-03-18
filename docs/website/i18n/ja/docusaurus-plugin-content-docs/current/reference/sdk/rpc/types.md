---
sidebar_position: 100
---

# 型定義

## RpcMethodDef\<I, O\>

[`rpc.method()`](/reference/sdk/rpc/method) で作成される型付き RPC メソッド定義です。

| フィールド | 型 | 必須 | 説明 |
|-------|------|----------|-------------|
| `description` | `string` | いいえ | 人間が読める説明 |
| `timeout` | `number` | いいえ | タイムアウト（ミリ秒単位） |
| `input` | `ZodType<I>` | いいえ | 入力バリデーション用の Zod スキーマ |
| `handler` | `(params: I) => Promise<O>` | はい | リクエストハンドラー |

## RpcHandlerFn\<O\>

`(params: unknown) => Promise<O>`

RPC メソッドハンドラーとして使用できるプレーンな非同期関数です。パースされた生の JSON ボディがバリデーションなしで直接渡されます。

## RpcMethodEntry

`RpcMethodDef | RpcHandlerFn`

[`rpc.serve()`](/reference/sdk/rpc/serve) で受け入れられるユニオン型です。同じ `methods` マップ内で `rpc.method()` による定義とプレーン関数を混在させることができます。

## RpcServeOptions

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `methods` | `Record<string, RpcMethodEntry>` | メソッド名から定義へのマップ |

## RpcServer

[`rpc.serve()`](/reference/sdk/rpc/serve) の戻り値です。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `port` | `number` | サーバーが待ち受けているポート |
| `close` | `() => Promise<void>` | サーバーをグレースフルにシャットダウンします |

## RpcCallOptions

[`rpc.call()`](/reference/sdk/rpc/call) のオプションです。

| フィールド | 型 | 必須 | 説明 |
|-------|------|----------|-------------|
| `modName` | `string` | はい | ターゲット MOD の名前 |
| `method` | `string` | はい | 呼び出す RPC メソッド名 |
| `body` | `unknown` | いいえ | メソッドハンドラーに渡すオプションのリクエストボディ |

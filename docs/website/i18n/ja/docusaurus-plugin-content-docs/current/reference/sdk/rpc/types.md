---
sidebar_position: 100
---

# 型定義

## RpcMethodDef\<I, O\>

[`rpc.method()`](./method) で作成される型付き RPC メソッド定義です。

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

[`rpc.serve()`](./serve) で受け入れられるユニオン型です。同じ `methods` マップ内で `rpc.method()` による定義とプレーン関数を混在させることができます。

## RpcServeOptions

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `methods` | `Record<string, RpcMethodEntry>` | メソッド名から定義へのマップ |

## RpcServer

[`rpc.serve()`](./serve) の戻り値です。

| フィールド | 型 | 説明 |
|-------|------|-------------|
| `port` | `number` | サーバーが待ち受けているポート |
| `close` | `() => Promise<void>` | サーバーをグレースフルにシャットダウンします |

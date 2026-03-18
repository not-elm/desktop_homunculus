---
sidebar_position: 1
---

# rpc

MOD サービスから RPC メソッドを定義・提供します。他の MOD、エンジン、AI エージェントがエンジンの RPC プロキシを通じてこれらのメソッドを呼び出すことができます。

:::note
このモジュールは Node.js API（`node:http`、`process`）を使用しており、ブラウザ環境では動作しません。メインの `@hmcs/sdk` エントリーポイントからは意図的に再エクスポートされていません。
:::

## インポート

```typescript
import { rpc } from "@hmcs/sdk/rpc";
```

## 関数

| 関数               | 説明                                                      |
| ------------------ | --------------------------------------------------------- |
| [serve](/reference/sdk/rpc/serve)   | RPC サーバーを起動し、メソッドをエンジンに登録            |
| [method](/reference/sdk/rpc/method) | オプションの Zod バリデーション付きで RPC メソッドを定義   |
| [call](/reference/sdk/rpc/call)     | 別の MOD サービスの RPC メソッドを呼び出す                 |

## 型定義

| 型 | 説明 |
|------|-------------|
| [RpcServer](/reference/sdk/rpc/types#rpcserver) | `serve()` が返すサーバーインスタンス |
| [RpcMethodEntry](/reference/sdk/rpc/types#rpcmethodentry) | `serve({ methods })` で使用するメソッド定義 |
| [RpcCallOptions](/reference/sdk/rpc/types#rpccalloptions) | `call()` のオプション |

---
sidebar_position: 4
---

# call

エンジンの HTTP API 経由で MOD サービスの RPC メソッドを呼び出します。ブラウザ（WebView）と Node.js の両方の環境で動作します。

## シグネチャ

```typescript
rpc.call<T = unknown>(options: RpcCallOptions): Promise<T>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `options.modName` | `string` | ターゲット MOD の名前 |
| `options.method` | `string` | 呼び出す RPC メソッド名 |
| `options.body` | `unknown` | メソッドハンドラーに渡すオプションのリクエストボディ |

## 戻り値

`Promise<T>` — MOD メソッドハンドラーからのパース済み JSON レスポンス。

## エラーレスポンス

エンジンプロキシが返す可能性のあるエラー：

| ステータス | 条件 |
|--------|-----------|
| 404 | ターゲット MOD にメソッドが見つからない |
| 502 | MOD サービスに到達不能 |
| 503 | MOD が未登録（サービスが実行されていない） |
| 504 | リクエストがタイムアウト |

## 使用例

```typescript
import { rpc } from "@hmcs/sdk/rpc";

// リクエストボディ付きで呼び出し
const result = await rpc.call<{ greeting: string }>({
  modName: "voicevox",
  method: "speak",
  body: { text: "Hello!" },
});
console.log(result.greeting);

// ボディなしで呼び出し
const status = await rpc.call<{ running: boolean }>({
  modName: "voicevox",
  method: "status",
});
```

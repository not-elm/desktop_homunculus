---
title: "host"
sidebar_position: 16
---

# host

:::warning
ほとんどの開発者は、HTTP API を直接呼び出す代わりに、高レベルのモジュール API（`entities`、`Vrm`、`audio` など）を使用してください。このモジュールは、SDK ラッパーがまだ存在しない高度なユースケース向けです。
:::

`localhost:3100` で動作する Desktop Homunculus エンジン API に直接アクセスするための低レベル HTTP クライアントです。`host` 名前空間は、他のすべての SDK モジュールが内部的に使用しています。

## インポート

```typescript
import { host } from "@hmcs/sdk";
```

## 設定

デフォルトでは SDK は `http://localhost:3100` に接続します。エンジンが別のポートで動作している場合は、ベース URL をオーバーライドしてください：

```typescript
host.configure({ baseUrl: "http://localhost:4000" });
```

## URL の構築

`host.createUrl(path, params?)` は、オプションのクエリパラメータ付きの完全な API URL を構築します。

```typescript
const url = host.createUrl("vrm");
// http://localhost:3100/vrm

const url = host.createUrl("entities", { name: "MyCharacter", root: 42 });
// http://localhost:3100/entities?name=MyCharacter&root=42
```

## リクエストの実行

すべてのリクエストメソッドは、非 OK レスポンスで自動的に `HomunculusApiError` をスローします。

```typescript
// GET
const response = await host.get(host.createUrl("vrm"));
const vrms = await response.json();

// JSON ボディ付き POST
await host.post(host.createUrl("vrm"), { asset: "my-mod:character" });

// PUT
await host.put(host.createUrl("vrm/123/state"), { state: "idle" });

// PATCH
await host.patch(host.createUrl("vrm/123/persona"), { profile: "cheerful" });

// DELETE
await host.deleteMethod(host.createUrl("vrm/123"));
```

## ストリーミング（NDJSON）

`host.postStream<T>(url, body?, signal?)` は POST リクエストを送信し、パースされた NDJSON オブジェクトが到着するたびに yield する非同期ジェネレータを返します。

```typescript
import { host, type HomunculusStreamError } from "@hmcs/sdk";

const stream = host.postStream<{ type: string; data: string }>(
  host.createUrl("commands/execute"),
  { command: "build" },
);

for await (const event of stream) {
  console.log(event);
}
```

## エラー処理

SDK は 2 つのエラークラスをエクスポートします：

### HomunculusApiError

HTTP API が非 OK ステータス（400 以上）を返した場合にスローされます。

```typescript
import { HomunculusApiError } from "@hmcs/sdk";

try {
  await host.get(host.createUrl("vrm/999"));
} catch (err) {
  if (err instanceof HomunculusApiError) {
    console.error(err.statusCode); // 404
    console.error(err.endpoint);   // リクエスト URL
    console.error(err.body);       // レスポンスボディのテキスト
  }
}
```

### HomunculusStreamError

NDJSON ストリームの行が JSON としてパースできない場合にスローされます。

```typescript
import { HomunculusStreamError } from "@hmcs/sdk";

// err.rawLine にはパースできない行が含まれます
```

シャドウオーバーレイパネルモジュールについては、[Shadow Panel](./shadow-panel) を参照してください。

## 次のステップ

- **[SDK 概要](./)** -- 完全なモジュールマップとクイック例

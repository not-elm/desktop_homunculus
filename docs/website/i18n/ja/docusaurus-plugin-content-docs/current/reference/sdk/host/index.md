---
sidebar_position: 1
---

# host

:::warning
ほとんどの開発者は、HTTP API を直接呼び出す代わりに、高レベルのモジュール API（`entities`、`Vrm`、`audio` など）を使用してください。このモジュールは、SDK ラッパーがまだ存在しない高度なユースケース向けです。
:::

`localhost:3100` で動作する Desktop Homunculus エンジン API に直接アクセスするための低レベル HTTP クライアントです。`host` 名前空間は、他のすべての SDK モジュールが内部的に使用しています。

## インポート

```typescript
import { host, HomunculusApiError, HomunculusStreamError } from "@hmcs/sdk";
```

## 関数一覧

| 関数 | 説明 |
|------|------|
| [configure](./configure) | エンジン API のベース URL を上書きする |
| [base](./base) | 現在のベース URL 文字列を返す |
| [baseUrl](./baseUrl) | 現在のベース URL を `URL` オブジェクトとして返す |
| [createUrl](./createUrl) | オプションのクエリパラメータ付きで完全な API URL を構築する |
| [get](./get) | GET リクエストを実行する |
| [post](./post) | JSON ボディ付きで POST リクエストを実行する |
| [put](./put) | JSON ボディ付きで PUT リクエストを実行する |
| [patch](./patch) | JSON ボディ付きで PATCH リクエストを実行する |
| [deleteMethod](./deleteMethod) | DELETE リクエストを実行する |
| [postStream](./postStream) | POST して NDJSON レスポンスを非同期ジェネレータとしてストリームする |

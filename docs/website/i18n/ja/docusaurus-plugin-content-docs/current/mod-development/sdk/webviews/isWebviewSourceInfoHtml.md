---
title: "isWebviewSourceInfoHtml"
sidebar_position: 27
---

# isWebviewSourceInfoHtml

`WebviewSourceInfo` がインライン HTML ソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceInfoHtml(source: WebviewSourceInfoHtml): source is WebviewSourceInfoHtml
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSourceInfoHtml` | 確認するソース情報 |

## 戻り値

`source.type === "html"` の場合に `true` を返し、型を `WebviewSourceInfoHtml` に絞り込みます。

## 例

```typescript
const info = await webview.info();

if (isWebviewSourceInfoHtml(info.source as WebviewSourceInfoHtml)) {
  console.log(info.source.content); // リストレスポンスでは undefined の場合あり
}
```

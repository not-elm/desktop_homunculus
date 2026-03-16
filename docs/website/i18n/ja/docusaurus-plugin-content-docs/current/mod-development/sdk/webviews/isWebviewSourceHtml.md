---
title: "isWebviewSourceHtml"
sidebar_position: 24
---

# isWebviewSourceHtml

`WebviewSource` がインライン HTML ソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceHtml(source: WebviewSource): source is WebviewSourceHtml
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSource` | 確認するソース |

## 戻り値

`source.type === "html"` の場合に `true` を返し、型を `WebviewSourceHtml` に絞り込みます。

## 例

```typescript
const source = webviewSource.html("<h1>Hello</h1>");

if (isWebviewSourceHtml(source)) {
  console.log(source.content); // "<h1>Hello</h1>"
}
```

---
title: "isWebviewSourceUrl"
sidebar_position: 23
---

# isWebviewSourceUrl

`WebviewSource` が URL ソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceUrl(source: WebviewSource): source is WebviewSourceUrl
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSource` | 確認するソース |

## 戻り値

`source.type === "url"` の場合に `true` を返し、型を [`WebviewSourceUrl`](./types#webviewsourceurl) に絞り込みます。

## 例

```typescript
const source = webviewSource.url("https://example.com");

if (isWebviewSourceUrl(source)) {
  console.log(source.url); // "https://example.com"
}
```

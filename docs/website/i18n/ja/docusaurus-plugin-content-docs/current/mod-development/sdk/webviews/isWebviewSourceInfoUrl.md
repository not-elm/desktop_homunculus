---
title: "isWebviewSourceInfoUrl"
sidebar_position: 26
---

# isWebviewSourceInfoUrl

`WebviewSourceInfo` が URL ソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceInfoUrl(source: WebviewSourceInfo): source is WebviewSourceInfoUrl
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSourceInfo` | 確認するソース情報 |

## 戻り値

`source.type === "url"` の場合に `true` を返し、型を `WebviewSourceInfoUrl` に絞り込みます。

## 例

```typescript
const info = await webview.info();

if (isWebviewSourceInfoUrl(info.source)) {
  console.log(info.source.url);
}
```

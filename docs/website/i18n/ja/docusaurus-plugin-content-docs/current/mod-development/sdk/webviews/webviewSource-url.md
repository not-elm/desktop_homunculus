---
title: "webviewSource.url"
sidebar_position: 20
---

# webviewSource.url

リモート URL からコンテンツを読み込む URL ソースを作成します。

```typescript
function url(url: string): WebviewSourceUrl
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `url` | `string` | URL 文字列 |

## 戻り値

`WebviewSourceUrl` オブジェクト：`{ type: "url", url }`。

## 例

```typescript
const source = webviewSource.url("https://example.com");
// { type: "url", url: "https://example.com" }
```

---
title: "webviewSource.html"
sidebar_position: 21
---

# webviewSource.html

HTML 文字列を直接レンダリングするインライン HTML ソースを作成します。

```typescript
function html(content: string): WebviewSourceHtml
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `content` | `string` | HTML 文字列 |

## 戻り値

`WebviewSourceHtml` オブジェクト：`{ type: "html", content }`。

## 例

```typescript
const source = webviewSource.html("<h1>Hello</h1>");
// { type: "html", content: "<h1>Hello</h1>" }
```

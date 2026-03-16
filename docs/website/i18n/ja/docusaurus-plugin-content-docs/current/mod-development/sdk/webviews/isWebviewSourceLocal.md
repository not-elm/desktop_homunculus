---
title: "isWebviewSourceLocal"
sidebar_position: 22
---

# isWebviewSourceLocal

`WebviewSource` がローカルアセットソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceLocal(source: WebviewSource): source is WebviewSourceLocal
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSource` | 確認するソース |

## 戻り値

`source.type === "local"` の場合に `true` を返し、型を `WebviewSourceLocal` に絞り込みます。

## 例

```typescript
const source = webviewSource.local("my-mod:ui");

if (isWebviewSourceLocal(source)) {
  console.log(source.id); // "my-mod:ui"
}
```

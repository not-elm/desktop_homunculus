---
title: "isWebviewSourceInfoLocal"
sidebar_position: 25
---

# isWebviewSourceInfoLocal

`WebviewSourceInfo` がローカルアセットソースかどうかを確認する型ガードです。

```typescript
function isWebviewSourceInfoLocal(source: WebviewSourceInfo): source is WebviewSourceInfoLocal
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `source` | `WebviewSourceInfo` | 確認するソース情報 |

## 戻り値

`source.type === "local"` の場合に `true` を返し、型を `WebviewSourceInfoLocal` に絞り込みます。

## 例

```typescript
const info = await webview.info();

if (isWebviewSourceInfoLocal(info.source)) {
  console.log(info.source.id);
}
```

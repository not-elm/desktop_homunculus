---
title: "webviewSource.local"
sidebar_position: 19
---

# webviewSource.local

MOD の `package.json` のアセットで宣言された HTML ファイルを指定するローカルアセットソースを作成します。

```typescript
function local(id: string): WebviewSourceLocal
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `id` | `string` | アセット ID（例：`"menu:ui"`、`"settings:ui"`） |

## 戻り値

[`WebviewSourceLocal`](./types#webviewsourcelocal) オブジェクト：`{ type: "local", id }`。

## 例

```typescript
const source = webviewSource.local("my-mod:ui");
// { type: "local", id: "my-mod:ui" }
```

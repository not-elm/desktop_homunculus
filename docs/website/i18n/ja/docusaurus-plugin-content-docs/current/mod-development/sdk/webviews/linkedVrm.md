---
title: "linkedVrm"
sidebar_position: 16
---

# linkedVrm

この WebView にリンクされた VRM を取得します。

```typescript
async linkedVrm(): Promise<Vrm | undefined>
```

## 戻り値

リンクされた `Vrm` インスタンス、または VRM がリンクされていない場合は `undefined` に解決される `Promise`。

## 例

```typescript
import { Vrm } from "@hmcs/sdk";

// リンクされた VRM を照会
const linked = await webview.linkedVrm();
// linked は Vrm インスタンス、またはリンクされていない場合は undefined
```

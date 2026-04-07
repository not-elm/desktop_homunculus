---
title: "linkedPersona"
sidebar_position: 16
---

# linkedPersona

この WebView にリンクされたペルソナを取得します。

```typescript
async linkedPersona(): Promise<Persona | undefined>
```

## 戻り値

リンクされた `Persona` インスタンス、またはペルソナがリンクされていない場合は `undefined` に解決される `Promise`。

## 例

```typescript
import { persona } from "@hmcs/sdk";

// リンクされたペルソナを照会
const linked = await webview.linkedPersona();
// linked は Persona インスタンス、またはリンクされていない場合は undefined
```

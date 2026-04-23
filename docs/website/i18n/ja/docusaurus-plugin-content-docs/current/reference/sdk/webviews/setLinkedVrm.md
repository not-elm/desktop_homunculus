---
title: "setLinkedPersona"
sidebar_position: 17
---

# setLinkedPersona

WebView をペルソナにリンクして、ペルソナのキャラクター位置に追従させます。

```typescript
async setLinkedPersona(personaId: string): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `personaId` | `string` | この WebView にリンクするペルソナ ID |

## 例

```typescript
import { persona } from "@hmcs/sdk";

const p = await persona.load("alice");

// リンク
await webview.setLinkedPersona(p.id);
```

リンクを解除するには [`unlinkPersona()`](./unlinkVrm) を使用してください。

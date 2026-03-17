---
title: "setLinkedVrm"
sidebar_position: 17
---

# setLinkedVrm

WebView を VRM キャラクターにリンクして、キャラクターの位置に追従させます。

```typescript
async setLinkedVrm(vrm: Vrm): Promise<void>
```

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `vrm` | `Vrm` | この WebView にリンクする VRM インスタンス |

## 例

```typescript
import { Vrm } from "@hmcs/sdk";

const vrm = await Vrm.findByName("MyAvatar");

// リンク
await webview.setLinkedVrm(vrm);
```

リンクを解除するには [`unlinkVrm()`](./unlinkVrm) を使用してください。

---
sidebar_position: 3
---

# input.parseMenu

メニューコマンドの stdin をパースし、リンクされたキャラクターの `Vrm` インスタンスを返します。メニューコマンドはメニュー UI から `{ "linkedVrm": <entityId> }` を stdin で受け取ります。

## パラメーター

なし。

## 戻り値

`Promise<Vrm>`

## 例

```typescript
import { input } from "@hmcs/sdk/commands";

const vrm = await input.parseMenu();
await vrm.setExpressions({ happy: 1.0 });
```

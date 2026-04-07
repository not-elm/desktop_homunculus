---
sidebar_position: 3
---

# input.parseMenu

メニューコマンドの stdin をパースし、リンクされたキャラクターの `Persona` インスタンスを返します。メニューコマンドはメニュー UI から `{ "linkedPersona": "<personaId>" }` を stdin で受け取ります。

## パラメーター

なし。

## 戻り値

`Promise<Persona>`

## 例

```typescript
import { input } from "@hmcs/sdk/commands";

const persona = await input.parseMenu();
await persona.vrm().setExpressions({ happy: 1.0 });
```

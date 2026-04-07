---
sidebar_position: 3
---

# input.parseMenu

Parse menu command stdin and return a `Persona` instance for the linked character. Menu commands receive `{ "linkedPersona": "<personaId>" }` on stdin from the menu UI.

## Parameters

None.

## Returns

`Promise<Persona>`

## Example

```typescript
import { input } from "@hmcs/sdk/commands";

const persona = await input.parseMenu();
await persona.vrm().setExpressions({ happy: 1.0 });
```

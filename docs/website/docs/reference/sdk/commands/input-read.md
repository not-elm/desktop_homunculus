---
sidebar_position: 4
---

# input.read

Read all of stdin as a raw UTF-8 string. Useful when you need the raw string without JSON parsing or validation.

## Parameters

None.

## Returns

`Promise<string>`

## Example

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
console.log("Received:", raw);
```

---
sidebar_position: 1
---

# assets

Query the asset registry -- list and filter assets by type and MOD. Assets are declared in each MOD's `package.json` and referenced by globally unique IDs using the format `"mod-name:asset-name"`.

## Import

```typescript
import { assets } from "@hmcs/sdk";
```

## Functions

| Function | Description |
|----------|-------------|
| [list](./list) | Returns all registered assets, optionally filtered by type and/or MOD name |

See also: [Type Definitions](./types)

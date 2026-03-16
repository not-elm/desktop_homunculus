---
sidebar_position: 1
---

# assets

アセットレジストリへのクエリ -- アセットの種類や MOD でフィルタリングして一覧を取得します。アセットは各 MOD の `package.json` で宣言され、`"mod-name:asset-name"` 形式のグローバルに一意な ID で参照されます。

## インポート

```typescript
import { assets } from "@hmcs/sdk";
```

## Functions

| Function | 説明 |
|----------|-------------|
| [list](./list) | 登録されたすべてのアセットを返す（オプションで種類や MOD 名によるフィルタリング可能） |

See also: [型定義](./types)

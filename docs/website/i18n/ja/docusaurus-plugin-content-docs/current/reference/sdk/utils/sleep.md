---
sidebar_position: 2
---

# sleep

`ms` ミリ秒後に解決する非ブロッキングの遅延です。

## パラメータ

| パラメータ | 型 | 説明 |
|-----------|------|-------------|
| `ms` | `number` | 待機するミリ秒数 |

## 戻り値

`Promise<void>`

## 例

```typescript
import { sleep } from "@hmcs/sdk";

await sleep(1000); // 1秒待機
```

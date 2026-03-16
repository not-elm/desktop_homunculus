---
sidebar_position: 4
---

# input.read

stdin 全体を生の UTF-8 文字列として読み取ります。JSON パースやバリデーションなしに生の文字列が必要な場合に便利です。

## パラメーター

なし。

## 戻り値

`Promise<string>`

## 例

```typescript
import { input } from "@hmcs/sdk/commands";

const raw = await input.read();
console.log("受信:", raw);
```

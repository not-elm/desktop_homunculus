---
title: "setExpressions"
sidebar_position: 28
---

# setExpressions

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setExpressions(weights)` は現在の表情オーバーライドを**すべて**置換します。レコードに含まれない表情は VRMA アニメーションの制御に戻ります。

```typescript
const character = await Vrm.findByName("MyAvatar");

// happy と blink をオーバーライド -- 他のすべての表情はアニメーションに戻る
await character.setExpressions({ happy: 1.0, blink: 0.5 });
```

:::tip
どの表情をオーバーライドするか完全に制御したい場合は `setExpressions` を使用してください。他のオーバーライドを変更せずに変更をレイヤリングしたい場合は [`modifyExpressions`](./modifyExpressions) を使用してください。
:::

## 例：感情リアクションシーケンス

```typescript
const character = await Vrm.findByName("MyAvatar");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

// 驚きリアクション
await character.setExpressions({ surprised: 1.0 });
await sleep(1000);

// 嬉しい表情に移行
await character.setExpressions({ happy: 1.0 });
await sleep(2000);

// アニメーション制御に戻す
await character.clearExpressions();
```

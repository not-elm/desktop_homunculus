---
title: "Vrm.findAllDetailed"
sidebar_position: 7
---

# Vrm.findAllDetailed

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.findAllDetailed()` は読み込まれているすべての VRM の完全なランタイム状態を返します -- トランスフォーム、表情、アニメーション、ペルソナ、リンクされた WebView を含みます。

```typescript
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: state=${s.state}`);
  console.log(`  Position: (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
  console.log(`  Animations: ${s.animations.length} active`);
  console.log(`  Expressions: ${s.expressions.expressions.length} defined`);
}
```

[`VrmSnapshot`](./types#vrmsnapshot) オブジェクトの配列を返します。詳細な状態なしに `Vrm` インスタンスのみが必要な場合は [`Vrm.findAll`](./findAll) を使用してください。

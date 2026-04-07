---
title: "Vrm.spawn"
sidebar_position: 2
---

# Vrm.spawn

```typescript
import { Vrm } from "@hmcs/sdk";
```

`Vrm.spawn(asset, options?)` は MOD アセット ID から新しい VRM キャラクターを作成し、スポーンされたエンティティにバインドされた `Vrm` インスタンスを返します。

```typescript
const character = await Vrm.spawn("my-mod:character");
```

## オプション

オプションオブジェクトを渡して初期トランスフォームとペルソナを設定できます：

```typescript
import { Vrm, type Persona } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character", {
  transform: {
    translation: [0, 0.5, 0],
    scale: [1.2, 1.2, 1.2],
    rotation: [0, 0, 0, 1],
  },
  persona: {
    profile: "A cheerful virtual assistant who loves to help.",
    personality: "Curious and open-minded, speaks with enthusiasm",
    metadata: {},
  },
});
```

`transform` フィールドは部分的な `TransformArgs` を受け付けます -- オーバーライドしたいフィールドのみ指定すれば大丈夫です。未指定のフィールドはエンジンのデフォルト値が使用されます。

```typescript
// 位置のみ設定し、スケールと回転はデフォルトのまま
const character = await Vrm.spawn("my-mod:character", {
  transform: { translation: [2, 0, 0] },
});
```

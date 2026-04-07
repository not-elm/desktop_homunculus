---
title: "persona"
sidebar_position: 38
---

# persona

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.persona()` はキャラクターの現在のペルソナデータを返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const persona = await character.persona();

console.log("Profile:", persona.profile);
console.log("Personality:", persona.personality);
console.log("Metadata:", persona.metadata);
```

プロフィール説明、性格テキスト、拡張メタデータを含む [`Persona`](./types#persona) オブジェクトを返します。更新するには [`setPersona`](./setPersona) を使用してください。

---
title: "setState"
sidebar_position: 13
---

# setState

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.setState(state)` はキャラクターの状態文字列を設定します。

```typescript
const character = await Vrm.findByName("MyAvatar");
await character.setState("custom-state");
```

状態は自由形式の文字列です。エンジンが使用するビルトイン状態は `"idle"`、`"drag"`、`"sitting"` です。状態を設定すると、開いているすべての [`VrmEventSource`](./events) で `state-change` イベントがトリガーされます。

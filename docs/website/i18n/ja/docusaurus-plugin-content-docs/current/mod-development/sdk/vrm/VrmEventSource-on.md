---
title: "VrmEventSource.on"
sidebar_position: 33
---

# VrmEventSource.on

```typescript
import { Vrm } from "@hmcs/sdk";
```

`eventSource.on(event, callback)` は [`VrmEventSource`](./types#vrmeventsource) にイベントリスナーを登録します。コールバックは同期でも非同期でもかまいません。

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();

eventSource.on("state-change", (e) => {
  console.log("New state:", e.state);
});

eventSource.on("pointer-click", async (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]})`);
  console.log(`Button: ${e.button}`);
});
```

## 例：状態マシン

キャラクターの状態に基づいてアニメーションと動作を駆動する一般的なパターンです：

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

const animOption = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

await character.playVrma({ asset: "vrma:idle-maid", ...animOption });

character.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    await character.playVrma({ asset: "vrma:idle-maid", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  } else if (e.state === "drag") {
    await character.unlook();
    await character.playVrma({
      asset: "vrma:grabbed",
      ...animOption,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    await character.playVrma({ asset: "vrma:idle-sitting", ...animOption });
    await sleep(500);
    await character.lookAtCursor();
  }
});
```

利用可能なイベントタイプとペイロードの完全なリストは [`events`](./events) を参照してください。

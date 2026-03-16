---
title: "events"
sidebar_position: 32
---

# events

```typescript
import { Vrm } from "@hmcs/sdk";
```

`vrm.events()` はキャラクターの Server-Sent Events（SSE）ストリームに接続された `VrmEventSource` を返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();
```

イベントリスナーを登録するには [`VrmEventSource.on`](./VrmEventSource-on) を使用し、接続を閉じるには [`VrmEventSource.close`](./VrmEventSource-close) を使用します。

`VrmEventSource` は TypeScript の `using` 宣言で使用するための `Disposable` プロトコルを実装しています：

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // このブロックの終わりで eventSource は自動的に閉じられます
}
```

## 利用可能なイベント

| イベント               | ペイロード                      | 説明                                         |
| ------------------- | ---------------------------- | --------------------------------------------------- |
| `state-change`      | `{ state: string }`          | キャラクターの状態が変更（idle、drag、sitting など） |
| `expression-change` | `{ state: string }`          | 表情が変更                                  |
| `vrma-play`         | `{ state: string }`          | VRMA アニメーションの再生開始                      |
| `vrma-finish`       | `{ state: string }`          | VRMA アニメーションの完了                             |
| `pointer-click`     | `{ globalViewport, button }` | キャラクターがクリックされた                               |
| `pointer-press`     | `{ globalViewport, button }` | キャラクター上でマウスボタンが押された                   |
| `pointer-release`   | `{ globalViewport, button }` | キャラクター上でマウスボタンが離された                  |
| `pointer-over`      | `{ globalViewport }`         | マウスがキャラクター領域に入った                        |
| `pointer-out`       | `{ globalViewport }`         | マウスがキャラクター領域を離れた                           |
| `pointer-move`      | `{ globalViewport }`         | キャラクター領域内でマウスが移動した                   |
| `pointer-cancel`    | `{ globalViewport }`         | ポインタインタラクションがキャンセルされた                       |
| `drag-start`        | `{ globalViewport }`         | ドラッグ開始                                        |
| `drag`              | `{ globalViewport, delta }`  | ドラッグ中（カーソルの差分を含む）        |
| `drag-end`          | `{ globalViewport }`         | ドラッグ終了                                          |
| `persona-change`    | `{ persona }`                | ペルソナデータが更新された                            |

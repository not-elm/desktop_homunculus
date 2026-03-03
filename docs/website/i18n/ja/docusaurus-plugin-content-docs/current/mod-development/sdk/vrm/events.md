---
title: "イベント"
sidebar_position: 6
---

# イベント

Server-Sent Events（SSE）を使用してリアルタイムのキャラクターイベントを購読します。イベントシステムにより、MOD はポインタインタラクション、状態変更、アニメーションイベント、ドラッグ操作、ペルソナ更新に反応できます。

## インポート

```typescript
import { Vrm } from "@hmcs/sdk";
```

## イベントソースの作成

`vrm.events()` はキャラクターの SSE ストリームに接続された `VrmEventSource` を返します。

```typescript
const character = await Vrm.findByName("MyAvatar");
const eventSource = character.events();
```

### Disposable サポート

`VrmEventSource` は `Disposable` プロトコルを実装しています。TypeScript の `using` 宣言を使用すると、変数がスコープ外になったときに自動的に接続を閉じます：

```typescript
{
  using eventSource = character.events();
  eventSource.on("state-change", (e) => {
    console.log("State:", e.state);
  });
  // このブロックの終わりで eventSource は自動的に閉じられます
}
```

`using` を使用しない場合は、手動で `close()` を呼び出してください：

```typescript
const eventSource = character.events();
// ... リスナーを登録 ...

// 完了したら：
eventSource.close();
```

## リスナーの登録

`.on(event, callback)` を使用してイベントハンドラを登録します。コールバックは同期でも非同期でもかまいません。

```typescript
const eventSource = character.events();

eventSource.on("state-change", (e) => {
  console.log("New state:", e.state);
});

eventSource.on("pointer-click", async (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]})`);
  console.log(`Button: ${e.button}`);
});
```

## イベントタイプ

### 状態イベント

| イベント | ペイロード | 説明 |
|---|---|---|
| `state-change` | `{ state: string }` | キャラクターの状態が変更（例：`"idle"`、`"drag"`、`"sitting"`） |
| `expression-change` | `{ state: string }` | 表情が変更 |

### アニメーションイベント

| イベント | ペイロード | 説明 |
|---|---|---|
| `vrma-play` | `{ state: string }` | VRMA アニメーションの再生開始 |
| `vrma-finish` | `{ state: string }` | VRMA アニメーションの完了 |

### ポインタイベント

| イベント | ペイロード | 説明 |
|---|---|---|
| `pointer-click` | `{ globalViewport, button }` | キャラクターがクリックされた |
| `pointer-press` | `{ globalViewport, button }` | キャラクター上でマウスボタンが押された |
| `pointer-release` | `{ globalViewport, button }` | キャラクター上でマウスボタンが離された |
| `pointer-over` | `{ globalViewport }` | マウスがキャラクター領域に入った |
| `pointer-out` | `{ globalViewport }` | マウスがキャラクター領域を離れた |
| `pointer-move` | `{ globalViewport }` | キャラクター領域内でマウスが移動した |
| `pointer-cancel` | `{ globalViewport }` | ポインタインタラクションがキャンセルされた |

### ドラッグイベント

| イベント | ペイロード | 説明 |
|---|---|---|
| `drag-start` | `{ globalViewport }` | ドラッグ開始 |
| `drag` | `{ globalViewport, delta }` | ドラッグ中。`delta` は前回のイベントからのカーソル移動量です。 |
| `drag-end` | `{ globalViewport }` | ドラッグ終了 |

### ペルソナイベント

| イベント | ペイロード | 説明 |
|---|---|---|
| `persona-change` | `{ persona }` | ペルソナデータが更新された（プロフィール、OCEAN、メタデータ） |

## イベントペイロード

`globalViewport` フィールドはグローバル画面座標（最も左のスクリーン端を原点とするマルチモニター座標）でのカーソル位置を表す `[number, number]` タプルです。

マウスイベントには `"Primary"`、`"Secondary"`、または `"Middle"` の値を持つ `button` フィールドが含まれます。

ドラッグイベントには前回のイベントからのカーソル移動量を表す `[number, number]` タプルの `delta` フィールドが含まれます。

## 例：状態マシン

イベントを使用してキャラクターの状態に基づいてアニメーションと動作を駆動する一般的なパターンです。これはビルトインの Elmer MOD で使用されるパターンです：

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

const animOption = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

// アイドルアニメーションで開始
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

## 例：クリックカウンター

```typescript
const character = await Vrm.findByName("MyAvatar");
let clickCount = 0;

const eventSource = character.events();

eventSource.on("pointer-click", (e) => {
  if (e.button === "Primary") {
    clickCount++;
    console.log(`Clicked ${clickCount} times`);
  }
});

eventSource.on("pointer-over", () => {
  console.log("Mouse hovering over character");
});

eventSource.on("pointer-out", () => {
  console.log("Mouse left character");
});
```

## 型定義

```typescript
class VrmEventSource implements Disposable {
  on<K extends keyof EventMap>(
    event: K,
    callback: (event: EventMap[K]) => void | Promise<void>,
  ): void;
  close(): void;
}

interface VrmPointerEvent {
  globalViewport: [number, number];
}

interface VrmDragEvent extends VrmPointerEvent {
  delta: [number, number];
}

interface VrmMouseEvent extends VrmPointerEvent {
  button: "Primary" | "Secondary" | "Middle";
}

interface VrmStateChangeEvent {
  state: string;
}

interface PersonaChangeEvent {
  persona: Persona;
}
```

## 次のステップ

- **[スポーンと検索](./spawn-and-find)** -- イベントをアタッチするキャラクターを作成・検索します。
- **[ペルソナ](./persona)** -- `persona-change` イベントをリッスンします。
- **[VRM 概要](./)** -- 完全な API リファレンス表。

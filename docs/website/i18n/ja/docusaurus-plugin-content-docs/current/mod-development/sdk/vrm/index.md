---
title: "VRM"
sidebar_position: 1
---

# VRM モジュール

`Vrm` クラスは `@hmcs/sdk` のコアモジュールです。VRM 3D キャラクターの完全なライフサイクルを管理します -- スポーン、検索、アニメーション、表情制御、ポインタ／ドラッグイベントの処理、リップシンクスピーチなどを提供します。

```typescript
import { Vrm } from "@hmcs/sdk";
```

## キャラクターのスポーン

`Vrm.spawn()` を使用して MOD アセットから新しい VRM キャラクターを作成します。慣例として、アセット ID は `"mod-name:asset-name"` 形式を使用します。

```typescript
import { Vrm } from "@hmcs/sdk";

// 基本的なスポーン
const character = await Vrm.spawn("my-mod:character");

// 初期トランスフォームとペルソナを指定してスポーン
const character = await Vrm.spawn("my-mod:character", {
  transform: {
    translation: [0, 0, 0],
    scale: [1, 1, 1],
  },
  persona: {
    profile: "A cheerful virtual assistant",
    ocean: { openness: 0.8, extraversion: 0.7 },
    metadata: {},
  },
});
```

`Vrm.spawn()` はスポーンされたエンティティにバインドされた `Vrm` インスタンスを返します。以降の操作はすべてこのインスタンスを使用します。

## 既存キャラクターの検索

キャラクターがすでにスポーンされている場合（例：別の MOD や以前のセッションで）、名前で検索したり、すべてのインスタンスを一覧表示したりできます。

```typescript
// VRM モデル名で検索
const character = await Vrm.findByName("MyAvatar");

// キャラクターの読み込みを待機（準備完了までブロック）
const character = await Vrm.waitLoadByName("MyAvatar");

// 読み込み済みのすべての VRM インスタンスを取得
const allCharacters = await Vrm.findAll();

// すべての VRM の詳細スナップショットを取得（状態、トランスフォーム、表情、アニメーション）
const snapshots = await Vrm.findAllDetailed();
for (const s of snapshots) {
  console.log(`${s.name}: ${s.state} at (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
}
```

## アニメーションの再生

`playVrma()` を使用してキャラクター上で VRMA アニメーションを再生します。アニメーションはアセット ID で参照します。ビルトインの `@hmcs/assets` MOD はデフォルトのアニメーションを提供します：`vrma:idle-maid`、`vrma:grabbed`、`vrma:idle-sitting`。

```typescript
import { Vrm, repeat } from "@hmcs/sdk";

const character = await Vrm.spawn("my-mod:character");

// 0.5秒のクロスフェードでループするアイドルアニメーションを再生
await character.playVrma({
  asset: "vrma:idle-maid",
  repeat: repeat.forever(),
  transitionSecs: 0.5,
});

// ワンショットアニメーションを再生（1回再生後に停止）
await character.playVrma({
  asset: "my-mod:wave",
  repeat: repeat.never(),
});

// アニメーションをN回再生
await character.playVrma({
  asset: "my-mod:nod",
  repeat: repeat.count(3),
});

// ワンショットアニメーションの完了を待ってから続行
await character.playVrma({
  asset: "my-mod:dance",
  repeat: repeat.never(),
  waitForCompletion: true,
});

// トランジション中にスプリングボーンをリセットして髪/衣服の揺れを防止
await character.playVrma({
  asset: "vrma:grabbed",
  repeat: repeat.forever(),
  resetSpringBones: true,
});
```

アニメーション状態の照会、アニメーションの停止、再生速度の制御も可能です：

```typescript
// すべてのアクティブなアニメーションを取得
const animations = await character.listVrma();

// 特定のアニメーションの状態を確認
const state = await character.vrmaState("vrma:idle-maid");
console.log(`Playing: ${state.playing}, Elapsed: ${state.elapsedSecs}s`);

// 再生速度を変更
await character.setVrmaSpeed("vrma:idle-maid", 1.5);

// アニメーションを停止
await character.stopVrma("vrma:idle-maid");
```

## キャラクターイベント

`events()` を使用してキャラクターからのリアルタイムイベントを購読します。キャラクターの SSE ストリームに接続する Server-Sent Events 接続を開きます。

```typescript
const character = await Vrm.spawn("my-mod:character");
const eventSource = character.events();

// 状態変更：idle、drag、sitting など
eventSource.on("state-change", async (e) => {
  console.log("New state:", e.state);
});

// ポインタインタラクション
eventSource.on("pointer-click", (e) => {
  console.log(`Clicked at (${e.globalViewport[0]}, ${e.globalViewport[1]}), button: ${e.button}`);
});

// ドラッグイベント
eventSource.on("drag-start", (e) => console.log("Drag started"));
eventSource.on("drag", (e) => console.log(`Dragging, delta: ${e.delta}`));
eventSource.on("drag-end", (e) => console.log("Drag ended"));

// ホバーイベント
eventSource.on("pointer-over", (e) => console.log("Mouse entered character"));
eventSource.on("pointer-out", (e) => console.log("Mouse left character"));

// アニメーションイベント
eventSource.on("vrma-play", (e) => console.log("Animation started:", e.state));
eventSource.on("vrma-finish", (e) => console.log("Animation finished:", e.state));

// ペルソナ変更
eventSource.on("persona-change", (e) => {
  console.log("Persona updated:", e.persona.profile);
});

// 完了したらイベントストリームを閉じる
eventSource.close();
```

`VrmEventSource` は `Disposable` を実装しているため、TypeScript 5.2+ の `using` で使用できます：

```typescript
using eventSource = character.events();
eventSource.on("state-change", (e) => { /* ... */ });
```

### 利用可能なイベント

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

## 主要な API

### ライフサイクル

| メソッド                       | 説明                                                                              |
| ---------------------------- | ---------------------------------------------------------------------------------------- |
| `Vrm.spawn(asset, options?)` | MOD アセット ID から新しい VRM をスポーンします。`Vrm` インスタンスを返します。                           |
| `Vrm.findByName(name)`       | モデル名で VRM を検索します。見つからない場合はスローします。                                       |
| `Vrm.waitLoadByName(name)`   | VRM の読み込み完了を待ってから返します。                                        |
| `Vrm.findAll()`              | 読み込み済みのすべての VRM インスタンスを `Vrm[]` として取得します。                                                 |
| `Vrm.findAllEntities()`      | 読み込み済みのすべての VRM エンティティ ID を `number[]` として取得します。                                             |
| `Vrm.findAllDetailed()`      | すべての VRM の詳細スナップショット（状態、トランスフォーム、表情、アニメーション、ペルソナ）を取得します。 |
| `Vrm.stream(callback)`       | 既存および今後作成される VRM インスタンスをストリームします。                            |
| `vrm.despawn()`              | この VRM をシーンから削除します。                                                          |

### アニメーション

| メソッド                           | 説明                                                               |
| -------------------------------- | ------------------------------------------------------------------------- |
| `vrm.playVrma(options)`          | リピート、トランジション、完了オプション付きで VRMA アニメーションを再生します。    |
| `vrm.stopVrma(asset)`            | アセット ID で指定した VRMA アニメーションを停止します。                               |
| `vrm.listVrma()`                 | この VRM にアタッチされたすべての VRMA アニメーションを一覧表示します。                            |
| `vrm.vrmaState(asset)`           | 特定のアニメーションの再生状態（再生中、速度、経過時間）を取得します。 |
| `vrm.setVrmaSpeed(asset, speed)` | アニメーションの再生速度を変更します。                                |

### 表情

| メソッド                           | 説明                                                             |
| -------------------------------- | ----------------------------------------------------------------------- |
| `vrm.expressions()`              | すべての表情とその現在のウェイトを取得します。                          |
| `vrm.setExpressions(weights)`    | 表情のウェイトを設定し、以前のすべてのオーバーライドを置換します。               |
| `vrm.modifyExpressions(weights)` | 表情のウェイトを部分的に更新します（他のオーバーライドは維持）。           |
| `vrm.clearExpressions()`         | すべての表情オーバーライドをクリアし、VRMA アニメーションに制御を戻します。    |
| `vrm.modifyMouth(weights)`       | リップシンク用の口の表情を設定します（口以外のオーバーライドは維持）。 |

### 視線追従（Look-At）

| メソッド                     | 説明                                        |
| -------------------------- | -------------------------------------------------- |
| `vrm.lookAtCursor()`       | キャラクターの目をマウスカーソルに追従させます。 |
| `vrm.lookAtTarget(entity)` | キャラクターの目を特定のエンティティに向けます。      |
| `vrm.unlook()`             | 視線追従動作を無効にします。                      |

### スピーチ

| メソッド                                              | 説明                                                               |
| --------------------------------------------------- | ------------------------------------------------------------------------- |
| `vrm.speakWithTimeline(audio, keyframes, options?)` | WAV オーディオをフレーム同期された表情キーフレームで再生し、リップシンクを行います。 |

### 状態と位置

| メソッド                | 説明                                                           |
| --------------------- | --------------------------------------------------------------------- |
| `vrm.state()`         | 現在の状態文字列を取得します（例："idle"、"drag"、"sitting"）。       |
| `vrm.setState(state)` | キャラクターの状態を設定します。                                            |
| `vrm.position()`      | 画面座標（`globalViewport`）とワールド座標の両方で位置を取得します。 |
| `vrm.name()`          | VRM モデル名を取得します。                                               |

### ペルソナ

| メソッド                    | 説明                                                                 |
| ------------------------- | --------------------------------------------------------------------------- |
| `vrm.persona()`           | キャラクターのペルソナ（プロフィール、性格、OCEAN 特性、メタデータ）を取得します。 |
| `vrm.setPersona(persona)` | キャラクターのペルソナデータを設定します。                                           |

### 物理

| メソッド                              | 説明                                                       |
| ----------------------------------- | ----------------------------------------------------------------- |
| `vrm.springBones()`                 | すべてのスプリングボーン（spring bone）チェーン（髪、衣服の物理）を取得します。              |
| `vrm.springBone(chainId)`           | エンティティ ID で特定のスプリングボーンチェーンを取得します。                    |
| `vrm.setSpringBone(chainId, props)` | スプリングボーンの物理プロパティ（剛性、抵抗、重力）を更新します。 |

### イベント

| メソッド                     | 説明                                                    |
| -------------------------- | -------------------------------------------------------------- |
| `vrm.events()`             | リアルタイムイベントストリーミング用の `VrmEventSource` を開きます。         |
| `vrm.findBoneEntity(bone)` | 名前付きボーン（例："head"、"leftHand"）のエンティティ ID を取得します。 |

## 完全な使用例

以下は `@hmcs/elmer` MOD の完全なサービスコードです。キャラクターのスポーン、状態に基づくアニメーション再生、カーソル追従を示しています。

```typescript
import { type TransformArgs, Vrm, preferences, repeat } from "@hmcs/sdk";

// プリファレンスからキャラクターの最後の位置を読み込む
const transform = await preferences.load<TransformArgs>("transform::elmer:vrm");

// VRM アセットを使用して Elmer キャラクターをスポーン
const elmer = await Vrm.spawn("elmer:vrm", {
  transform,
});

// 非同期ディレイのヘルパー
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

// 共通のアニメーションオプション：0.5秒のクロスフェードで永続ループ
const option = {
  repeat: repeat.forever(),
  transitionSecs: 0.5,
} as const;

// アイドルアニメーションで開始
await elmer.playVrma({
  asset: "vrma:idle-maid",
  ...option,
});

// キャラクターの状態変更に反応
elmer.events().on("state-change", async (e) => {
  if (e.state === "idle") {
    // アイドル時：アイドルアニメーションを再生し、カーソルを追従
    await elmer.playVrma({
      asset: "vrma:idle-maid",
      ...option,
    });
    await sleep(500);
    await elmer.lookAtCursor();
  } else if (e.state === "drag") {
    // ドラッグ中：カーソル追従を停止し、つかまれアニメーションを再生
    await elmer.unlook();
    await elmer.playVrma({
      asset: "vrma:grabbed",
      ...option,
      resetSpringBones: true,
    });
  } else if (e.state === "sitting") {
    // 座っている時：座りアニメーションを再生
    await elmer.playVrma({
      asset: "vrma:idle-sitting",
      ...option,
    });
    await sleep(500);
    await elmer.lookAtCursor();
  }
});
```

## 次のステップ

- **[SDK 概要](../)** -- すべての SDK モジュールとその説明の完全なリスト。

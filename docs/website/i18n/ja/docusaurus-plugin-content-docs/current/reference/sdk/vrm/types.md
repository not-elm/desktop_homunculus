---
title: "型定義"
sidebar_position: 100
---

# 型定義

`@hmcs/sdk` の `vrm` モジュールからエクスポートされるすべての型です。

```typescript
import { Vrm, repeat, VrmEventSource } from "@hmcs/sdk";
```

## コア

### `Bones`

```typescript
type Bones =
  | "hips" | "spine" | "chest" | "neck" | "head"
  | "leftShoulder" | "leftArm" | "leftForeArm" | "leftHand"
  | "rightShoulder" | "rightArm" | "rightForeArm" | "rightHand"
  | "leftUpLeg" | "leftLeg" | "leftFoot"
  | "rightUpLeg" | "rightLeg" | "rightFoot";
```

### `PositionResponse`

```typescript
interface PositionResponse {
  /** グローバル画面座標（最も左のスクリーンを原点とするマルチモニター）。表示されていない場合は null。 */
  globalViewport: [number, number] | null;
  /** Bevy ワールド座標。 */
  world: Vec3;
}
```

### `SpawnVrmOptions`

```typescript
interface SpawnVrmOptions {
  transform?: TransformArgs;
  persona?: Persona;
}
```

### `VrmSnapshot`

```typescript
interface VrmSnapshot {
  entity: number;
  name: string;
  state: string;
  transform: Transform;
  globalViewport: [number, number] | null;
  expressions: ExpressionsResponse;
  animations: VrmaInfo[];
  lookAt: LookAtState | null;
  linkedWebviews: number[];
  persona: Persona;
}
```

### `VrmMetadata`

```typescript
interface VrmMetadata {
  name: string;
  entity: number;
}
```

### `MoveToArgs`

```typescript
/** VRM をビューポート位置に移動するための引数。 */
interface MoveToArgs {
  globalViewport: GlobalViewport;
}
```

### `LookAtState`

```typescript
type LookAtState =
  | { type: "cursor" }
  | { type: "target"; entity: number };
```

## ペルソナ

### `Persona`

```typescript
interface Persona {
  /** キャラクターのプロフィール／背景説明。 */
  profile: string;
  /** 自然言語による性格の説明。 */
  personality?: string | null;
  /** ビッグファイブ性格パラメータ。 */
  ocean: Ocean;
  /** MOD 用の拡張メタデータ。 */
  metadata: Record<string, unknown>;
}
```

### `Ocean`

```typescript
interface Ocean {
  openness?: number;
  conscientiousness?: number;
  extraversion?: number;
  agreeableness?: number;
  neuroticism?: number;
}
```

## 表情

### `ExpressionsResponse`

```typescript
interface ExpressionsResponse {
  expressions: ExpressionInfo[];
}
```

### `ExpressionInfo`

```typescript
interface ExpressionInfo {
  name: string;
  weight: number;
  isBinary: boolean;
  overrideBlink: OverrideType;
  overrideLookAt: OverrideType;
  overrideMouth: OverrideType;
}
```

### `OverrideType`

```typescript
type OverrideType = "none" | "blend" | "block";
```

## アニメーション

### `VrmaInfo`

```typescript
interface VrmaInfo {
  entity: number;
  name: string;
  playing: boolean;
}
```

### `VrmaPlayRequest`

```typescript
interface VrmaPlayRequest {
  asset: string;
  transitionSecs?: number;
  repeat?: VrmaRepeat;
  waitForCompletion?: boolean;
  resetSpringBones?: boolean;
}
```

### `VrmaRepeat`

```typescript
interface VrmaRepeat {
  type: "forever" | "never" | "count";
  count?: number;
}
```

### `VrmaState`

```typescript
interface VrmaState {
  playing: boolean;
  repeat: string;
  speed: number;
  elapsedSecs: number;
}
```

### `VrmaSpeedBody`

```typescript
interface VrmaSpeedBody {
  asset: string;
  speed: number;
}
```

## スプリングボーン

### `SpringBoneChainsResponse`

```typescript
interface SpringBoneChainsResponse {
  chains: SpringBoneChain[];
}
```

### `SpringBoneChain`

```typescript
interface SpringBoneChain {
  entity: number;
  joints: string[];
  props: SpringBoneProps;
}
```

### `SpringBoneProps`

```typescript
interface SpringBoneProps {
  stiffness: number;
  dragForce: number;
  gravityPower: number;
  gravityDir: [number, number, number];
  hitRadius: number;
}
```

## スピーチ

### `TimelineKeyframe`

```typescript
interface TimelineKeyframe {
  /** このキーフレームの持続時間（秒）。 */
  duration: number;
  /** 設定する表情ウェイト。キーは表情名、値は 0.0-1.0。 */
  targets?: Record<string, number>;
}
```

### `SpeakTimelineOptions`

```typescript
interface SpeakTimelineOptions {
  /** true の場合、スピーチ完了を待ちます。デフォルトは true。 */
  waitForCompletion?: boolean;
  /** 隣接するキーフレーム間のスムースステップブレンディングの秒数。デフォルトは 0.05。 */
  transitionDuration?: number;
}
```

## イベント

### `VrmEventSource`

```typescript
class VrmEventSource implements Disposable {
  on<K extends keyof EventMap>(
    event: K,
    callback: (event: EventMap[K]) => void | Promise<void>,
  ): void;
  close(): void;
  [Symbol.dispose](): void;
}
```

### `EventMap`

```typescript
type EventMap = {
  "drag-start": VrmPointerEvent;
  "drag": VrmDragEvent;
  "drag-end": VrmPointerEvent;
  "pointer-press": VrmMouseEvent;
  "pointer-click": VrmMouseEvent;
  "pointer-release": VrmMouseEvent;
  "pointer-over": VrmPointerEvent;
  "pointer-out": VrmPointerEvent;
  "pointer-cancel": VrmPointerEvent;
  "pointer-move": VrmPointerEvent;
  "state-change": VrmStateChangeEvent;
  "expression-change": VrmStateChangeEvent;
  "vrma-play": VrmStateChangeEvent;
  "vrma-finish": VrmStateChangeEvent;
  "persona-change": PersonaChangeEvent;
};
```

### `VrmPointerEvent`

```typescript
interface VrmPointerEvent {
  globalViewport: [number, number];
}
```

### `VrmDragEvent`

```typescript
interface VrmDragEvent extends VrmPointerEvent {
  delta: [number, number];
}
```

### `Button`

```typescript
type Button = "Primary" | "Secondary" | "Middle";
```

### `VrmMouseEvent`

```typescript
interface VrmMouseEvent extends VrmPointerEvent {
  button: Button;
}
```

### `VrmStateChangeEvent`

```typescript
interface VrmStateChangeEvent {
  state: string;
}
```

### `PersonaChangeEvent`

```typescript
interface PersonaChangeEvent {
  persona: Persona;
}
```

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

```typescript
type Bones =
  | "hips" | "spine" | "chest" | "neck" | "head"
  | "leftShoulder" | "leftArm" | "leftForeArm" | "leftHand"
  | "rightShoulder" | "rightArm" | "rightForeArm" | "rightHand"
  | "leftUpLeg" | "leftLeg" | "leftFoot"
  | "rightUpLeg" | "rightLeg" | "rightFoot";

interface PositionResponse {
  /** グローバル画面座標（最も左のスクリーンを原点とするマルチモニター）。表示されていない場合は null。 */
  globalViewport: [number, number] | null;
  /** Bevy ワールド座標。 */
  world: Vec3;
}

interface SpawnVrmOptions {
  transform?: TransformArgs;
  persona?: Persona;
}

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

interface VrmMetadata {
  name: string;
  entity: number;
}

/** VRM をビューポート位置に移動するための引数。 */
interface MoveToArgs {
  globalViewport: GlobalViewport;
}

type LookAtState =
  | { type: "cursor" }
  | { type: "target"; entity: number };
```

## ペルソナ

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

interface Ocean {
  openness?: number;
  conscientiousness?: number;
  extraversion?: number;
  agreeableness?: number;
  neuroticism?: number;
}
```

## 表情

```typescript
interface ExpressionsResponse {
  expressions: ExpressionInfo[];
}

interface ExpressionInfo {
  name: string;
  weight: number;
  isBinary: boolean;
  overrideBlink: OverrideType;
  overrideLookAt: OverrideType;
  overrideMouth: OverrideType;
}

type OverrideType = "none" | "blend" | "block";
```

## アニメーション

```typescript
interface VrmaInfo {
  entity: number;
  name: string;
  playing: boolean;
}

interface VrmaPlayRequest {
  asset: string;
  transitionSecs?: number;
  repeat?: VrmaRepeat;
  waitForCompletion?: boolean;
  resetSpringBones?: boolean;
}

interface VrmaRepeat {
  type: "forever" | "never" | "count";
  count?: number;
}

interface VrmaState {
  playing: boolean;
  repeat: string;
  speed: number;
  elapsedSecs: number;
}

interface VrmaSpeedBody {
  asset: string;
  speed: number;
}
```

## スプリングボーン

```typescript
interface SpringBoneChainsResponse {
  chains: SpringBoneChain[];
}

interface SpringBoneChain {
  entity: number;
  joints: string[];
  props: SpringBoneProps;
}

interface SpringBoneProps {
  stiffness: number;
  dragForce: number;
  gravityPower: number;
  gravityDir: [number, number, number];
  hitRadius: number;
}
```

## スピーチ

```typescript
interface TimelineKeyframe {
  /** このキーフレームの持続時間（秒）。 */
  duration: number;
  /** 設定する表情ウェイト。キーは表情名、値は 0.0-1.0。 */
  targets?: Record<string, number>;
}

interface SpeakTimelineOptions {
  /** true の場合、スピーチ完了を待ちます。デフォルトは true。 */
  waitForCompletion?: boolean;
  /** 隣接するキーフレーム間のスムースステップブレンディングの秒数。デフォルトは 0.05。 */
  transitionDuration?: number;
}
```

## イベント

```typescript
class VrmEventSource implements Disposable {
  on<K extends keyof EventMap>(
    event: K,
    callback: (event: EventMap[K]) => void | Promise<void>,
  ): void;
  close(): void;
  [Symbol.dispose](): void;
}

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

interface VrmPointerEvent {
  globalViewport: [number, number];
}

interface VrmDragEvent extends VrmPointerEvent {
  delta: [number, number];
}

type Button = "Primary" | "Secondary" | "Middle";

interface VrmMouseEvent extends VrmPointerEvent {
  button: Button;
}

interface VrmStateChangeEvent {
  state: string;
}

interface PersonaChangeEvent {
  persona: Persona;
}
```

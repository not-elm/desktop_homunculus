---
title: "Type Definitions"
sidebar_position: 100
---

# Type Definitions

All types exported from the `vrm` module of `@hmcs/sdk`.

```typescript
import { Vrm, repeat, VrmEventSource } from "@hmcs/sdk";
```

## Core

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
  /** Global screen coordinates (multi-monitor origin at leftmost screen). Null if not visible. */
  globalViewport: [number, number] | null;
  /** Bevy world coordinates. */
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
/** Arguments for moving a VRM to a viewport position. */
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

## Persona

### `Persona`

```typescript
interface Persona {
  /** Character profile/background description. */
  profile: string;
  /** Personality description in natural language. */
  personality?: string | null;
  /** Big Five personality parameters. */
  ocean: Ocean;
  /** Extension metadata for MODs. */
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

## Expressions

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

## Animation

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

## Spring Bone

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

## Speech

### `TimelineKeyframe`

```typescript
interface TimelineKeyframe {
  /** Duration of this keyframe in seconds. */
  duration: number;
  /** Expression weights to set. Keys are expression names, values are 0.0-1.0. */
  targets?: Record<string, number>;
}
```

### `SpeakTimelineOptions`

```typescript
interface SpeakTimelineOptions {
  /** If true, waits for speech to complete. Defaults to true. */
  waitForCompletion?: boolean;
  /** Seconds for smoothstep blending between adjacent keyframes. Defaults to 0.05. */
  transitionDuration?: number;
}
```

## Events

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

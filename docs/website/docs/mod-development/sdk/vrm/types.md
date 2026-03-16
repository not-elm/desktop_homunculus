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

```typescript
type Bones =
  | "hips" | "spine" | "chest" | "neck" | "head"
  | "leftShoulder" | "leftArm" | "leftForeArm" | "leftHand"
  | "rightShoulder" | "rightArm" | "rightForeArm" | "rightHand"
  | "leftUpLeg" | "leftLeg" | "leftFoot"
  | "rightUpLeg" | "rightLeg" | "rightFoot";

interface PositionResponse {
  /** Global screen coordinates (multi-monitor origin at leftmost screen). Null if not visible. */
  globalViewport: [number, number] | null;
  /** Bevy world coordinates. */
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

/** Arguments for moving a VRM to a viewport position. */
interface MoveToArgs {
  globalViewport: GlobalViewport;
}

type LookAtState =
  | { type: "cursor" }
  | { type: "target"; entity: number };
```

## Persona

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

interface Ocean {
  openness?: number;
  conscientiousness?: number;
  extraversion?: number;
  agreeableness?: number;
  neuroticism?: number;
}
```

## Expressions

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

## Animation

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

## Spring Bone

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

## Speech

```typescript
interface TimelineKeyframe {
  /** Duration of this keyframe in seconds. */
  duration: number;
  /** Expression weights to set. Keys are expression names, values are 0.0-1.0. */
  targets?: Record<string, number>;
}

interface SpeakTimelineOptions {
  /** If true, waits for speech to complete. Defaults to true. */
  waitForCompletion?: boolean;
  /** Seconds for smoothstep blending between adjacent keyframes. Defaults to 0.05. */
  transitionDuration?: number;
}
```

## Events

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

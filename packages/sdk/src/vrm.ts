import { type Transform, type TransformArgs, type Vec3 } from "./math";
import { type GlobalViewport } from "./coordinates";
import { host } from "./host";
import { EventSource } from "eventsource";
import { entities } from "./entities";

// --- Persona types ---

/**
 * Big Five personality traits (OCEAN model).
 *
 * @example
 * ```typescript
 * const ocean: Ocean = {
 *   openness: 0.8,
 *   conscientiousness: 0.6,
 *   extraversion: 0.7,
 * };
 * ```
 */
export interface Ocean {
    /** Openness (0.0=conservative, 1.0=curious) */
    openness?: number;
    /** Conscientiousness (0.0=spontaneous, 1.0=organized) */
    conscientiousness?: number;
    /** Extraversion (0.0=introverted, 1.0=extroverted) */
    extraversion?: number;
    /** Agreeableness (0.0=independent, 1.0=cooperative) */
    agreeableness?: number;
    /** Neuroticism (0.0=stable, 1.0=sensitive) */
    neuroticism?: number;
}

/**
 * Persona data for a VRM character.
 *
 * @example
 * ```typescript
 * const persona: Persona = {
 *   profile: "A cheerful virtual assistant",
 *   personality: "Friendly and helpful",
 *   ocean: { openness: 0.8, extraversion: 0.7 },
 *   metadata: {},
 * };
 * ```
 */
export interface Persona {
    /** Character profile/background description. */
    profile: string;
    /** Personality description in natural language. */
    personality?: string | null;
    /** Big Five personality parameters. */
    ocean: Ocean;
    /** Extension metadata for MODs. */
    metadata: Record<string, unknown>;
}

// --- VRM types ---

/** Response for VRM state queries. */
export interface VrmStateResponse {
    state: string;
}

/** Request body for setting VRM state. */
export interface VrmStateRequest {
    state: string;
}

/** Override type for expression override settings. */
export type OverrideType = "none" | "blend" | "block";

/**
 * Information about a single VRM expression.
 *
 * @example
 * ```typescript
 * const vrm = await Vrm.findByName("MyCharacter");
 * const { expressions } = await vrm.expressions();
 * for (const expr of expressions) {
 *   console.log(`${expr.name}: weight=${expr.weight}, binary=${expr.isBinary}`);
 * }
 * ```
 */
export interface ExpressionInfo {
    /** Expression name (e.g. "happy", "aa", "blink"). */
    name: string;
    /** Current weight value (0.0-1.0). */
    weight: number;
    /** Whether this expression is binary (snaps to 0 or 1). */
    isBinary: boolean;
    /** Override type for blink expressions. */
    overrideBlink: OverrideType;
    /** Override type for lookAt expressions. */
    overrideLookAt: OverrideType;
    /** Override type for mouth expressions. */
    overrideMouth: OverrideType;
}

/** Response for VRM expression queries. */
export interface ExpressionsResponse {
    expressions: ExpressionInfo[];
}

/** Arguments for moving a VRM to a viewport position. */
export interface MoveToArgs {
    globalViewport: GlobalViewport;
}

/** Spring bone physics properties. */
export interface SpringBoneProps {
    stiffness: number;
    dragForce: number;
    gravityPower: number;
    gravityDir: [number, number, number];
    hitRadius: number;
}

/** A single spring bone chain. */
export interface SpringBoneChain {
    entity: number;
    joints: string[];
    props: SpringBoneProps;
}

/** Response for spring bone chains query. */
export interface SpringBoneChainsResponse {
    chains: SpringBoneChain[];
}

/** Repeat settings for VRMA playback. */
export interface VrmaRepeat {
    type: "forever" | "never" | "count";
    count?: number;
}

/**
 * Helper functions for building {@link VrmaRepeat} values.
 *
 * @example
 * ```typescript
 * await vrm.playVrma({
 *   asset: "vrma:idle-maid",
 *   repeat: repeat.forever(),
 * });
 *
 * await vrm.playVrma({
 *   asset: "vrma:grabbed",
 *   repeat: repeat.count(3),
 * });
 * ```
 */
export namespace repeat {
    /**
     * Repeat the animation forever.
     */
    export function forever(): VrmaRepeat {
        return { type: "forever" };
    }

    /**
     * Play the animation once (no repeat).
     */
    export function never(): VrmaRepeat {
        return { type: "never" };
    }

    /**
     * Repeat the animation a fixed number of times.
     *
     * @param n Positive integer repeat count.
     * @throws {RangeError} If `n` is not a positive integer.
     */
    export function count(n: number): VrmaRepeat {
        if (!Number.isInteger(n) || !Number.isFinite(n) || n <= 0) {
            throw new RangeError("repeat.count(n) requires a positive integer");
        }
        return { type: "count", count: n };
    }
}

/** Request body for playing a VRMA animation. */
export interface VrmaPlayRequest {
    asset: string;
    transitionSecs?: number;
    repeat?: VrmaRepeat;
    waitForCompletion?: boolean;
    /** If true, resets SpringBone velocities to prevent bouncing during animation transitions. */
    resetSpringBones?: boolean;
}

/** State of a VRMA animation. */
export interface VrmaState {
    playing: boolean;
    repeat: string;
    speed: number;
    elapsedSecs: number;
}

/** Info about a VRMA animation entity. */
export interface VrmaInfo {
    entity: number;
    name: string;
    playing: boolean;
}

/** Current look-at state of a VRM. */
export type LookAtState =
    | { type: "cursor" }
    | { type: "target"; entity: number };

/**
 * Snapshot of a VRM instance with full runtime state.
 *
 * @example
 * ```typescript
 * const snapshots = await Vrm.findAllDetailed();
 * for (const s of snapshots) {
 *   console.log(`${s.name}: ${s.state} at (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
 * }
 * ```
 */
export interface VrmSnapshot {
    /** The asset ID of the VRM model (e.g. "elmer:model"). Null if not tracked. */
    assetId: string | null;
    entity: number;
    name: string;
    state: string;
    transform: Transform;
    globalViewport: GlobalViewport | null;
    expressions: ExpressionsResponse;
    animations: VrmaInfo[];
    lookAt: LookAtState | null;
    linkedWebviews: number[];
    persona: Persona;
}

/**
 * Response from the VRM position endpoint.
 *
 * @example
 * ```ts
 * const vrm = await Vrm.findByName("MyCharacter");
 * const pos = await vrm.position();
 * console.log(`Screen: (${pos.globalViewport?.[0]}, ${pos.globalViewport?.[1]})`);
 * console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
 * ```
 */
export interface PositionResponse {
    /** Global screen coordinates (multi-monitor origin at leftmost screen). Null if not visible. */
    globalViewport: GlobalViewport | null;
    /** Bevy world coordinates. */
    world: Vec3;
}

/** Request body for setting VRMA playback speed. */
export interface VrmaSpeedBody {
    asset: string;
    speed: number;
}

export interface SpawnVrmOptions {
    transform?: TransformArgs;
    persona?: Persona;
}

/**
 * A single keyframe in a speech timeline.
 */
export interface TimelineKeyframe {
    /**
     * Duration of this keyframe in seconds.
     */
    duration: number;
    /**
     * Expression targets to set during this keyframe.
     * Keys are expression names (e.g. "aa", "ih", "happy"), values are weights (0.0-1.0).
     */
    targets?: Record<string, number>;
}

/**
 * Options for the timeline speech API.
 */
export interface SpeakTimelineOptions {
    /**
     * If true, the method will wait for the speech to complete.
     * Defaults to true.
     */
    waitForCompletion?: boolean;
    /**
     * Duration in seconds for smoothstep blending between adjacent keyframes.
     * Defaults to 0.05 (50ms). Clamped to 40% of each keyframe's duration.
     */
    transitionDuration?: number;
}

export interface VrmPointerEvent {
    /**
     * The cursor position in the global viewport.
     */
    globalViewport: [number, number];
}

export interface VrmDragEvent extends VrmPointerEvent {
    /**
     * The change in cursor position since the last event.
     */
    delta: [number, number];
}

export type Button = "Primary" | "Secondary" | "Middle";

export interface VrmMouseEvent extends VrmPointerEvent {
    /**
     * The button that was pressed or released.
     */
    button: Button;
}

export interface VrmStateChangeEvent {
    /**
     * The new state of the VRM.
     */
    state: string;
}

export interface PersonaChangeEvent {
    /**
     * The updated persona.
     */
    persona: Persona;
}

export type EventMap = {
    "drag-start": VrmPointerEvent;
    drag: VrmDragEvent;
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

export interface VrmMetadata {
    name: string;
    entity: number;
}

export type Bones =
    | "hips"
    | "spine"
    | "chest"
    | "neck"
    | "head"
    | "leftShoulder"
    | "leftArm"
    | "leftForeArm"
    | "leftHand"
    | "rightShoulder"
    | "rightArm"
    | "rightForeArm"
    | "rightHand"
    | "leftUpLeg"
    | "leftLeg"
    | "leftFoot"
    | "rightUpLeg"
    | "rightLeg"
    | "rightFoot";

export class VrmEventSource implements Disposable {
    constructor(readonly eventSource: EventSource) { }

    /**
     * Registers an event listener for the specified event type.
     */
    on<K extends keyof EventMap>(
        event: K,
        callback: (event: EventMap[K]) => void | Promise<void>,
    ) {
        this.eventSource.addEventListener(event, (e) => {
            callback(JSON.parse(e.data) as EventMap[K]);
        });
    }

    /**
     * Closes the EventSource connection.
     */
    close() {
        this.eventSource.close();
    }

    [Symbol.dispose]() {
        this.eventSource.close();
    }
}

export class Vrm {
    /** The Bevy entity ID of the VRM instance. */
    readonly entity: number;

    /** The character ID, if this Vrm was created from a Character. */
    readonly characterId?: string;

    private _resolvePromise?: Promise<string>;

    constructor(entity: number, characterId?: string) {
        this.entity = entity;
        this.characterId = characterId;
    }

    /**
     * Returns an EventSource for receiving events related to this VRM entity.
     */
    async events(): Promise<VrmEventSource> {
        const url = host.createUrl(
            `characters/${await this.resolveCharacterId()}/vrm/events`,
        );
        return new VrmEventSource(new EventSource(url));
    }

    /**
     * Returns the current state of the VRM.
     *
     * @deprecated Use {@link Character.state} instead.
     */
    async state(): Promise<string> {
        const id = await this.resolveCharacterId();
        const response = await host.get(
            host.createUrl(`characters/${id}/state`),
        );
        const json = (await response.json()) as { state: string };
        return json.state;
    }

    /**
     * Sets the state of the VRM.
     *
     * @deprecated Use {@link Character.setState} instead.
     * @param state The new state to set.
     */
    async setState(state: string): Promise<void> {
        const id = await this.resolveCharacterId();
        await host.put(host.createUrl(`characters/${id}/state`), { state });
    }

    /**
     * Returns the persona of the VRM.
     *
     * @deprecated Use {@link Character.persona} instead.
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * const persona = await vrm.persona();
     * console.log(persona.profile);
     * ```
     */
    async persona(): Promise<Persona> {
        const id = await this.resolveCharacterId();
        const response = await host.get(
            host.createUrl(`characters/${id}/persona`),
        );
        return (await response.json()) as Persona;
    }

    /**
     * Sets the persona of the VRM.
     *
     * @deprecated Use {@link Character.setPersona} instead.
     * @param persona The persona data to set.
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * await vrm.setPersona({
     *   profile: "A cheerful assistant",
     *   ocean: { openness: 0.8, extraversion: 0.7 },
     *   metadata: {},
     * });
     * ```
     */
    async setPersona(persona: Persona): Promise<void> {
        const id = await this.resolveCharacterId();
        await host.put(host.createUrl(`characters/${id}/persona`), persona);
    }

    /**
     * Returns the name of the VRM character.
     */
    async name(): Promise<string> {
        return await entities.name(this.entity);
    }

    /**
     * Finds the entity ID of a bone by its name.
     */
    async findBoneEntity(bone: Bones): Promise<number> {
        const response = await this.fetch(bone);
        return Number(await response.json());
    }

    /**
     * Despawns this VRM entity.
     */
    async despawn(): Promise<void> {
        await this.delete("despawn");
    }

    /**
     * Gets the current position of this VRM in both screen and world coordinates.
     *
     * @example
     * ```ts
     * const vrm = await Vrm.findByName("MyCharacter");
     * const pos = await vrm.position();
     * console.log(`Screen: (${pos.globalViewport?.[0]}, ${pos.globalViewport?.[1]})`);
     * console.log(`World: (${pos.world[0]}, ${pos.world[1]}, ${pos.world[2]})`);
     * ```
     */
    async position(): Promise<PositionResponse> {
        const response = await this.fetch("position");
        return (await response.json()) as PositionResponse;
    }

    /**
     * Gets all expressions and their current weights, including metadata
     * such as binary status and override settings.
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * const { expressions } = await vrm.expressions();
     * for (const expr of expressions) {
     *   console.log(`${expr.name}: ${expr.weight}`);
     * }
     * ```
     */
    async expressions(): Promise<ExpressionsResponse> {
        const response = await this.fetch("expressions");
        return (await response.json()) as ExpressionsResponse;
    }

    /**
     * Sets expression weights, replacing all previous overrides.
     * Expressions not included will return to VRMA animation control.
     *
     * @param weights A record of expression names to weight values (0.0-1.0).
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * await vrm.setExpressions({ happy: 1.0, blink: 0.5 });
     * ```
     */
    async setExpressions(weights: Record<string, number>): Promise<void> {
        await this.put("expressions", { weights });
    }

    /**
     * Modifies specific expression weights without affecting others (partial update).
     * Existing overrides not mentioned remain unchanged.
     *
     * @param weights A record of expression names to weight values (0.0-1.0).
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * // Only modifies "happy", leaves other overrides intact
     * await vrm.modifyExpressions({ happy: 1.0 });
     * ```
     */
    async modifyExpressions(weights: Record<string, number>): Promise<void> {
        await this.patch("expressions", { weights });
    }

    /**
     * Clears all expression overrides, returning control to VRMA animation.
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * await vrm.clearExpressions();
     * ```
     */
    async clearExpressions(): Promise<void> {
        await this.delete("expressions");
    }

    /**
     * Modifies mouth expression weights for lip-sync.
     * Unspecified mouth expressions are reset to 0.0.
     * Non-mouth expression overrides are preserved.
     *
     * @param weights A record of mouth expression names to weight values (0.0-1.0).
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * await vrm.modifyMouth({ aa: 0.8, oh: 0.2 });
     * ```
     */
    async modifyMouth(weights: Record<string, number>): Promise<void> {
        await this.patch("expressions/mouth", { weights });
    }

    /**
     * Gets all spring bone chains.
     */
    async springBones(): Promise<SpringBoneChainsResponse> {
        const response = await this.fetch("spring-bones");
        return (await response.json()) as SpringBoneChainsResponse;
    }

    /**
     * Gets a single spring bone chain by entity ID.
     *
     * @param chainId The chain entity ID.
     */
    async springBone(chainId: number): Promise<SpringBoneChain> {
        const response = await this.fetch(`spring-bones/${chainId}`);
        return (await response.json()) as SpringBoneChain;
    }

    /**
     * Updates spring bone properties for a chain.
     *
     * @param chainId The chain entity ID.
     * @param props Partial properties to update.
     */
    async setSpringBone(
        chainId: number,
        props: Partial<SpringBoneProps>,
    ): Promise<void> {
        await this.put(`spring-bones/${chainId}`, props);
    }

    /**
     * Gets all VRMA animations for this VRM.
     */
    async listVrma(): Promise<VrmaInfo[]> {
        const response = await this.fetch("vrma");
        return (await response.json()) as VrmaInfo[];
    }

    /**
     * Plays a VRMA animation.
     *
     * @param options Play request options including asset, repeat, transition.
     */
    async playVrma(options: VrmaPlayRequest): Promise<void> {
        await this.post("vrma/play", options);
    }

    /**
     * Stops a VRMA animation.
     *
     * @param asset The asset ID of the VRMA animation to stop.
     */
    async stopVrma(asset: string): Promise<void> {
        await this.post("vrma/stop", { asset });
    }

    /**
     * Gets the state of a VRMA animation.
     *
     * @param asset The asset ID of the VRMA animation to query.
     */
    async vrmaState(asset: string): Promise<VrmaState> {
        const id = await this.resolveCharacterId();
        const response = await host.get(
            host.createUrl(`characters/${id}/vrm/vrma/state`, { asset }),
        );
        return (await response.json()) as VrmaState;
    }

    /**
     * Sets the playback speed of a VRMA animation.
     *
     * @param asset The asset ID of the VRMA animation.
     * @param speed The playback speed.
     */
    async setVrmaSpeed(asset: string, speed: number): Promise<void> {
        await this.put("vrma/speed", { asset, speed });
    }

    /**
     * Speaks using pre-generated audio with a timeline of expression keyframes.
     * This allows any TTS engine to be used — the engine receives WAV audio and
     * frame-synchronized lip-sync data.
     *
     * @param audio - WAV audio data as ArrayBuffer or Uint8Array.
     * @param keyframes - Timeline keyframes specifying expression targets and durations.
     * @param options - Optional settings (e.g. waitForCompletion).
     *
     * @example
     * ```typescript
     * const vrm = await Vrm.findByName("MyCharacter");
     * const wavData = await fetchWavFromTTS("Hello world");
     * await vrm.speakWithTimeline(wavData, [
     *   { duration: 0.1, targets: { aa: 1.0 } },
     *   { duration: 0.05 },
     *   { duration: 0.12, targets: { oh: 1.0, happy: 0.5 } },
     * ]);
     * ```
     */
    async speakWithTimeline(
        audio: ArrayBuffer | Uint8Array,
        keyframes: TimelineKeyframe[],
        options?: SpeakTimelineOptions,
    ): Promise<void> {
        const bytes = audio instanceof Uint8Array ? audio : new Uint8Array(audio);
        let binary = "";
        for (let i = 0; i < bytes.length; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        const base64Audio = btoa(binary);
        await this.post("speech/timeline", {
            audio: base64Audio,
            keyframes,
            ...options,
        });
    }

    /**
     * Looks at the mouse cursor.
     */
    async lookAtCursor() {
        await this.put("look/cursor");
    }

    /**
     * Sets the VRM's look-at target to a specific entity.
     *
     * @param target The entity ID to look at.
     */
    async lookAtTarget(target: number) {
        await this.put(`look/target/${target}`);
    }

    /**
     * Disables the VRM's look-at functionality.
     */
    async unlook() {
        await this.delete("look");
    }

    /**
     * Finds a VRM instance by its name.
     *
     * @deprecated Use {@link Character.find} instead for character-based lifecycle management.
     * @param vrmName VRM character name
     */
    static async findByName(vrmName: string): Promise<Vrm> {
        const response = await host.get(host.createUrl("vrm", { name: vrmName }));
        const entities = (await response.json()) as number[];
        if (entities.length === 0) {
            throw new Error(`VRM not found: ${vrmName}`);
        }
        return new Vrm(entities[0]);
    }

    /**
     * Waits for a VRM instance to be spawned and initialized by its name.
     *
     * @param vrmName VRM character name
     */
    static async waitLoadByName(vrmName: string): Promise<Vrm> {
        const response = await host.get(
            host.createUrl("vrm/wait-load", {
                name: vrmName,
            }),
        );
        return new Vrm(Number(await response.json()));
    }

    /**
     * Returns entity IDs of all currently loaded VRM instances.
     *
     * @example
     * ```typescript
     * const entities = await Vrm.findAllEntities();
     * console.log(`Found ${entities.length} VRM entities`);
     * ```
     */
    static async findAllEntities(): Promise<number[]> {
        const response = await host.get(host.createUrl("vrm"));
        return (await response.json()) as number[];
    }

    /**
     * Returns detailed snapshot of all VRM instances.
     *
     * @deprecated Use {@link Character.findAll} instead for character-based lifecycle management.
     *
     * @example
     * ```typescript
     * const snapshots = await Vrm.findAllDetailed();
     * for (const s of snapshots) {
     *   console.log(`${s.name}: ${s.state} at (${s.globalViewport?.[0]}, ${s.globalViewport?.[1]})`);
     * }
     * ```
     */
    static async findAllDetailed(): Promise<VrmSnapshot[]> {
        const response = await host.get(host.createUrl("vrm/snapshot"));
        return (await response.json()) as VrmSnapshot[];
    }

    static streamMetadata(
        f: (vrm: VrmMetadata) => void | Promise<void>,
    ): EventSource {
        const es = new EventSource(host.createUrl("vrm/stream"));
        es.addEventListener("message", (e) => {
            f(JSON.parse(e.data));
        });
        return es;
    }

    /**
     * Streams all currently existing VRM instances and any VRM instances that will be created in the future.
     * @param f
     */
    static stream(f: (vrm: Vrm) => void | Promise<void>): EventSource {
        return Vrm.streamMetadata((metadata) => {
            f(new Vrm(metadata.entity));
        });
    }

    /**
     * Returns all VRM instances that are currently loaded.
     */
    static async findAll(): Promise<Vrm[]> {
        const entities = await Vrm.findAllEntities();
        return entities.map((entity) => new Vrm(entity));
    }

    private async resolveCharacterId(): Promise<string> {
        if (this.characterId) return this.characterId;
        if (!this._resolvePromise) {
            this._resolvePromise = this.fetchCharacterId();
        }
        return this._resolvePromise;
    }

    private async fetchCharacterId(): Promise<string> {
        const response = await host.get(host.createUrl("characters"));
        const characters = (await response.json()) as Array<{
            id: string;
            entity: number;
        }>;
        const match = characters.find((c) => c.entity === this.entity);
        if (!match) {
            throw new Error(`No character found for entity ${this.entity}`);
        }
        return match.id;
    }

    private async characterVrmUrl(path: string): Promise<string> {
        const id = await this.resolveCharacterId();
        return `characters/${id}/vrm/${path}`;
    }

    private async fetch(path: string): Promise<Response> {
        return await host.get(
            host.createUrl(await this.characterVrmUrl(path)),
        );
    }

    private async post(path: string, body?: object): Promise<Response> {
        return await host.post(
            host.createUrl(await this.characterVrmUrl(path)),
            body,
        );
    }

    private async put(path: string, body?: object) {
        await host.put(
            host.createUrl(await this.characterVrmUrl(path)),
            body,
        );
    }

    private async patch(path: string, body?: object) {
        await host.patch(
            host.createUrl(await this.characterVrmUrl(path)),
            body,
        );
    }

    private async delete(path: string) {
        await host.deleteMethod(
            host.createUrl(await this.characterVrmUrl(path)),
        );
    }
}

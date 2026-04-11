import { EventSource } from 'eventsource';
import { host } from './host';
import type { Transform } from './math';

// --- Persona types ---

/** Gender identity for a persona. */
export type Gender = 'male' | 'female' | 'other' | 'unknown';

/**
 * Snapshot of a persona returned by the server.
 *
 * @example
 * ```typescript
 * const personas = await Persona.list();
 * for (const p of personas) {
 *   console.log(`${p.id}: ${p.name ?? "(unnamed)"} — ${p.profile}`);
 * }
 * ```
 */
export interface PersonaSnapshot {
  /** URL-safe unique identifier. */
  id: string;
  /** Optional display name / nickname. */
  name?: string | null;
  /** Age of the character. null means unknown. */
  age?: number | null;
  /** Gender identity. */
  gender: Gender;
  /** First-person pronoun (e.g. "watashi", "boku"). */
  firstPersonPronoun?: string | null;
  /** Character profile / background description. */
  profile: string;
  /** Free-text personality description for agent prompts. */
  personality?: string | null;
  /** Current ephemeral state (e.g. "idle", "sitting", "drag"). */
  state: string;
  /** The asset ID of the currently attached VRM, or null. */
  vrmAssetId?: string | null;
  /** Asset ID for the persona's thumbnail image, or null. */
  thumbnail?: string | null;
  /** Extension metadata for MODs. */
  metadata: Record<string, unknown>;
  /** Whether this persona currently has a spawned ECS entity. */
  spawned: boolean;
}

/**
 * Partial update payload for a persona.
 *
 * Only the fields you include will be updated; omitted fields are left unchanged.
 *
 * @example
 * ```typescript
 * const p = await Persona.load("alice");
 * await p.patch({ name: "Alice v2", personality: "Cheerful and energetic" });
 * ```
 */
export interface PatchPersona {
  /** New display name. */
  name?: string;
  /** New age. Pass `null` to clear (set to unknown). */
  age?: number | null;
  /** New gender. */
  gender?: Gender;
  /** New first-person pronoun. */
  firstPersonPronoun?: string;
  /** New profile text. */
  profile?: string;
  /** New personality description. */
  personality?: string;
  /** New VRM asset ID. */
  vrmAssetId?: string | null;
  /** New thumbnail asset ID. Pass `null` to clear. */
  thumbnail?: string | null;
  /** Replace all metadata. */
  metadata?: Record<string, unknown>;
}

/**
 * Full snapshot of a persona including transform and VRM state.
 *
 * Returned by the `GET /personas/snapshot` bulk endpoint.
 *
 * @example
 * ```typescript
 * // Full snapshots are returned by the bulk snapshot endpoint
 * const response = await host.get(host.createUrl("/personas/snapshot"));
 * const snapshots = await response.json() as PersonaFullSnapshot[];
 * ```
 */
export interface PersonaFullSnapshot {
  /** Persona ID. */
  id: string;
  /** Display name. */
  name: string | null;
  /** Age. */
  age: number | null;
  /** Gender identity. */
  gender: Gender;
  /** First-person pronoun. */
  firstPersonPronoun: string | null;
  /** Profile text. */
  profile: string;
  /** Personality description. */
  personality: string | null;
  /** Current state string. */
  state: string;
  /** World-space transform. */
  transform: Transform;
  /** Entity IDs of linked webviews. */
  linkedWebviews: string[];
  /** Attached VRM state, or null if no VRM is attached. */
  vrm: {
    assetId: string;
    expressions: Record<string, number>;
    animations: unknown[];
    lookAt: unknown | null;
    springBones: unknown;
  } | null;
}

// --- Event types ---

/**
 * Map of persona SSE event names to their payload types.
 */
export type PersonaEventMap = {
  /** Fired when persona data is modified. */
  'persona-change': { persona: PersonaSnapshot };
  /** Fired when the persona state changes. */
  'state-change': { state: string };
  /** A VRM model was attached to this persona. */
  'vrm-attached': { assetId: string };
  /** A VRM model was detached from this persona. */
  'vrm-detached': { assetId: string };
  /** Drag started on the VRM. */
  'drag-start': unknown;
  /** VRM is being dragged. */
  drag: unknown;
  /** Drag ended on the VRM. */
  'drag-end': unknown;
  /** Pointer button pressed on the VRM. */
  'pointer-press': unknown;
  /** Pointer click on the VRM. */
  'pointer-click': unknown;
  /** Pointer moved over the VRM. */
  'pointer-move': unknown;
  /** Pointer button released on the VRM. */
  'pointer-release': unknown;
  /** Pointer entered the VRM hit area. */
  'pointer-over': unknown;
  /** Pointer left the VRM hit area. */
  'pointer-out': unknown;
  /** Pointer interaction cancelled. */
  'pointer-cancel': unknown;
  /** VRM expression weights changed. */
  'expression-change': unknown;
  /** VRMA animation started playing. */
  'vrma-play': unknown;
  /** VRMA animation finished. */
  'vrma-finish': unknown;
};

/**
 * SSE event source for receiving persona-related events.
 *
 * Wraps a native `EventSource` and provides typed `on()` for each event kind.
 *
 * @example
 * ```typescript
 * const p = await Persona.load("alice");
 * const events = p.events();
 * events.on("state-change", (data) => {
 *   console.log("New state:", data.state);
 * });
 * // Later:
 * events.close();
 * ```
 */
export class PersonaEventSource implements Disposable {
  constructor(readonly eventSource: EventSource) {}

  /**
   * Registers an event listener for the specified event type.
   *
   * @param event - The event name to listen for
   * @param callback - Handler invoked with the parsed event payload
   * @returns `this` for chaining
   *
   * @example
   * ```typescript
   * const events = p.events();
   * events
   *   .on("pointer-click", (data) => console.log("clicked", data))
   *   .on("state-change", (data) => console.log("state:", data.state));
   * ```
   */
  on<K extends keyof PersonaEventMap>(
    event: K,
    callback: (event: PersonaEventMap[K]) => void | Promise<void>,
  ): this {
    this.eventSource.addEventListener(event, (e) => {
      callback(JSON.parse(e.data) as PersonaEventMap[K]);
    });
    return this;
  }

  /**
   * Closes the EventSource connection.
   */
  close(): void {
    this.eventSource.close();
  }

  [Symbol.dispose](): void {
    this.eventSource.close();
  }
}

// --- PersonaVrm class ---

/**
 * Accessor for VRM operations on a persona's attached VRM model.
 *
 * Obtained via {@link Persona.vrm}. All methods throw if no VRM is attached.
 *
 * @example
 * ```typescript
 * const p = await Persona.load("alice");
 * const vrm = p.vrm();
 * await vrm.setExpressions({ happy: 1.0 });
 * await vrm.playVrma({ asset: "vrma:idle-maid", repeat: { type: "forever" } });
 * ```
 */
export class PersonaVrm {
  constructor(private readonly personaId: string) {}

  private url(path: string): URL {
    return host.createUrl(`personas/${encodeURIComponent(this.personaId)}/vrm/${path}`);
  }

  /**
   * Gets all expressions and their current weights.
   *
   * @example
   * ```typescript
   * const vrm = p.vrm();
   * const exprs = await vrm.expressions();
   * console.log(exprs);
   * ```
   */
  async expressions(): Promise<Record<string, number>> {
    const response = await host.get(this.url('expressions'));
    return (await response.json()) as Record<string, number>;
  }

  /**
   * Modifies specific expression weights (merges with current weights).
   *
   * @param expressions - Expression name-to-weight map (0.0-1.0)
   *
   * @example
   * ```typescript
   * await p.vrm().setExpressions({ happy: 1.0, blink: 0.5 });
   * ```
   */
  async setExpressions(expressions: Record<string, number>): Promise<void> {
    await host.patch(this.url('expressions'), { weights: expressions });
  }

  /**
   * Clears all expression overrides, returning control to VRMA animation.
   *
   * @example
   * ```typescript
   * await p.vrm().clearExpressions();
   * ```
   */
  async clearExpressions(): Promise<void> {
    await host.deleteMethod(this.url('expressions'));
  }

  /**
   * Plays a VRMA animation on the persona's VRM.
   *
   * @param params - Playback options including asset ID, repeat, and transition settings
   *
   * @example
   * ```typescript
   * await p.vrm().playVrma({
   *   asset: "vrma:idle-maid",
   *   repeat: { type: "forever" },
   *   transitionSecs: 0.3,
   * });
   * ```
   */
  async playVrma(params: {
    asset: string;
    repeat?: unknown;
    transitionSecs?: number;
    resetSpringBones?: boolean;
    waitForCompletion?: boolean;
  }): Promise<void> {
    await host.post(this.url('vrma/play'), params);
  }

  /**
   * Stops a VRMA animation on the persona's VRM.
   *
   * @param asset - The asset ID of the animation to stop
   *
   * @example
   * ```typescript
   * await p.vrm().stopVrma("vrma:idle-maid");
   * ```
   */
  async stopVrma(asset: string): Promise<void> {
    await host.post(this.url('vrma/stop'), { asset });
  }

  /**
   * Lists all VRMA animations for the persona's VRM.
   *
   * @example
   * ```typescript
   * const animations = await p.vrm().listVrma();
   * console.log(animations);
   * ```
   */
  async listVrma(): Promise<unknown[]> {
    const response = await host.get(this.url('vrma'));
    return (await response.json()) as unknown[];
  }

  /**
   * Gets the position of the persona's VRM.
   *
   * @example
   * ```typescript
   * const pos = await p.vrm().position();
   * console.log(`World: ${pos.world}`);
   * ```
   */
  async position(): Promise<Transform> {
    const response = await host.get(this.url('position'));
    return (await response.json()) as Transform;
  }

  /**
   * Gets the entity ID of a specific bone in the persona's VRM.
   *
   * @param bone - Bone name (e.g. "head", "leftHand")
   *
   * @example
   * ```typescript
   * const boneEntity = await p.vrm().bone("head");
   * ```
   */
  async bone(bone: string): Promise<unknown> {
    const response = await host.get(this.url(`bone/${encodeURIComponent(bone)}`));
    return await response.json();
  }

  /**
   * Sets the VRM to look at the mouse cursor.
   *
   * @example
   * ```typescript
   * await p.vrm().lookAtCursor();
   * ```
   */
  async lookAtCursor(): Promise<void> {
    await host.put(this.url('look/cursor'));
  }

  /**
   * Sets the VRM to look at a specific entity.
   *
   * @param target - The entity ID to look at
   *
   * @example
   * ```typescript
   * await p.vrm().lookAtTarget("123");
   * ```
   */
  async lookAtTarget(target: string): Promise<void> {
    await host.put(this.url(`look/target/${encodeURIComponent(target)}`));
  }

  /**
   * Disables the VRM's look-at functionality.
   *
   * @example
   * ```typescript
   * await p.vrm().unlook();
   * ```
   */
  async unlook(): Promise<void> {
    await host.deleteMethod(this.url('look'));
  }

  /**
   * Gets all spring bone chains for the persona's VRM.
   *
   * @example
   * ```typescript
   * const bones = await p.vrm().springBones();
   * ```
   */
  async springBones(): Promise<unknown> {
    const response = await host.get(this.url('spring-bones'));
    return await response.json();
  }

  /**
   * Updates spring bone properties for a chain.
   *
   * @param chainId - The chain entity ID
   * @param props - Partial properties to update
   *
   * @example
   * ```typescript
   * await p.vrm().setSpringBones(42, { stiffness: 2.0 });
   * ```
   */
  async setSpringBones(chainId: number, props: unknown): Promise<void> {
    await host.patch(this.url(`spring-bones/${chainId}`), props);
  }

  /**
   * Speaks using pre-generated audio with a timeline of expression keyframes.
   *
   * @param audio - WAV audio data as ArrayBuffer or Uint8Array
   * @param keyframes - Timeline keyframes specifying expression targets and durations
   * @param options - Optional settings (e.g. waitForCompletion)
   *
   * @example
   * ```typescript
   * const wavData = await fetchWavFromTTS("Hello world");
   * await p.vrm().speakWithTimeline(wavData, [
   *   { duration: 0.1, targets: { aa: 1.0 } },
   *   { duration: 0.05 },
   *   { duration: 0.12, targets: { oh: 1.0, happy: 0.5 } },
   * ]);
   * ```
   */
  async speakWithTimeline(
    audio: ArrayBuffer | Uint8Array,
    keyframes: unknown,
    options?: { waitForCompletion?: boolean; transitionDuration?: number },
  ): Promise<void> {
    const bytes = audio instanceof Uint8Array ? audio : new Uint8Array(audio);
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    const base64Audio = btoa(binary);
    await host.post(this.url('speech/timeline'), {
      audio: base64Audio,
      keyframes,
      ...options,
    });
  }
}

// --- Persona class ---

/**
 * Represents a persona instance and provides methods to read/write its fields
 * and manage its attached VRM.
 *
 * Obtain an instance via {@link Persona.create}, {@link Persona.load}, or
 * from the list returned by {@link Persona.list}.
 *
 * @example
 * ```typescript
 * const p = await Persona.create({ id: "alice", name: "Alice", profile: "A cheerful assistant" });
 * console.log(await p.name()); // "Alice"
 * await p.setPersonality("Friendly and curious");
 * await p.attachVrm("elmer:model");
 * ```
 */
export class Persona {
  constructor(readonly id: string) {}

  private url(path?: string): URL {
    const base = `personas/${encodeURIComponent(this.id)}`;
    return host.createUrl(path ? `${base}/${path}` : base);
  }

  /**
   * Returns the full persona snapshot from the server.
   *
   * @example
   * ```typescript
   * const p = await Persona.load("alice");
   * const snap = await p.snapshot();
   * console.log(`${snap.id}: ${snap.name}`);
   * ```
   */
  async snapshot(): Promise<PersonaSnapshot> {
    const response = await host.get(this.url());
    return (await response.json()) as PersonaSnapshot;
  }

  /**
   * Gets the display name of the persona.
   *
   * @example
   * ```typescript
   * const name = await p.name();
   * console.log(name ?? "(unnamed)");
   * ```
   */
  async name(): Promise<string | null> {
    const response = await host.get(this.url('name'));
    const body = (await response.json()) as { name: string | null };
    return body.name;
  }

  /**
   * Sets the display name of the persona.
   *
   * @param name - The new display name
   *
   * @example
   * ```typescript
   * await p.setName("Alice v2");
   * ```
   */
  async setName(name: string): Promise<void> {
    await host.put(this.url('name'), { name });
  }

  /**
   * Gets the age of the persona.
   *
   * @example
   * ```typescript
   * const age = await p.age();
   * console.log(age ?? "unknown");
   * ```
   */
  async age(): Promise<number | null> {
    const response = await host.get(this.url('age'));
    const body = (await response.json()) as { age: number | null };
    return body.age;
  }

  /**
   * Sets the age of the persona.
   *
   * @param age - The new age
   *
   * @example
   * ```typescript
   * await p.setAge(25);
   * ```
   */
  async setAge(age: number): Promise<void> {
    await host.put(this.url('age'), { age });
  }

  /**
   * Gets the gender of the persona.
   *
   * @example
   * ```typescript
   * const gender = await p.gender();
   * console.log(gender); // "female"
   * ```
   */
  async gender(): Promise<Gender> {
    const response = await host.get(this.url('gender'));
    const body = (await response.json()) as { gender: Gender };
    return body.gender;
  }

  /**
   * Sets the gender of the persona.
   *
   * @param gender - The new gender
   *
   * @example
   * ```typescript
   * await p.setGender("female");
   * ```
   */
  async setGender(gender: Gender): Promise<void> {
    await host.put(this.url('gender'), { gender });
  }

  /**
   * Gets the first-person pronoun of the persona.
   *
   * @example
   * ```typescript
   * const pronoun = await p.firstPersonPronoun();
   * console.log(pronoun ?? "(auto)");
   * ```
   */
  async firstPersonPronoun(): Promise<string | null> {
    const response = await host.get(this.url('first-person-pronoun'));
    const body = (await response.json()) as { firstPersonPronoun: string | null };
    return body.firstPersonPronoun;
  }

  /**
   * Sets the first-person pronoun of the persona.
   *
   * @param pronoun - The new pronoun (e.g. "watashi", "boku")
   *
   * @example
   * ```typescript
   * await p.setFirstPersonPronoun("watashi");
   * ```
   */
  async setFirstPersonPronoun(pronoun: string): Promise<void> {
    await host.put(this.url('first-person-pronoun'), { firstPersonPronoun: pronoun });
  }

  /**
   * Gets the profile text of the persona.
   *
   * @example
   * ```typescript
   * const profile = await p.profile();
   * console.log(profile);
   * ```
   */
  async profile(): Promise<string> {
    const response = await host.get(this.url('profile'));
    const body = (await response.json()) as { profile: string };
    return body.profile;
  }

  /**
   * Sets the profile text of the persona.
   *
   * @param profile - The new profile description
   *
   * @example
   * ```typescript
   * await p.setProfile("A cheerful virtual assistant");
   * ```
   */
  async setProfile(profile: string): Promise<void> {
    await host.put(this.url('profile'), { profile });
  }

  /**
   * Gets the personality description of the persona.
   *
   * @example
   * ```typescript
   * const personality = await p.personality();
   * console.log(personality ?? "(default)");
   * ```
   */
  async personality(): Promise<string | null> {
    const response = await host.get(this.url('personality'));
    const body = (await response.json()) as { personality: string | null };
    return body.personality;
  }

  /**
   * Sets the personality description of the persona.
   *
   * @param personality - Free-text personality description
   *
   * @example
   * ```typescript
   * await p.setPersonality("Friendly and curious, speaks politely but with enthusiasm");
   * ```
   */
  async setPersonality(personality: string): Promise<void> {
    await host.put(this.url('personality'), { personality });
  }

  /**
   * Gets the extension metadata of the persona.
   *
   * @example
   * ```typescript
   * const meta = await p.metadata();
   * console.log(meta);
   * ```
   */
  async metadata(): Promise<Record<string, unknown>> {
    const response = await host.get(this.url('metadata'));
    const body = (await response.json()) as { metadata: Record<string, unknown> };
    return body.metadata;
  }

  /**
   * Replaces all extension metadata of the persona.
   *
   * @param metadata - The new metadata object
   *
   * @example
   * ```typescript
   * await p.setMetadata({ favoriteColor: "blue", level: 5 });
   * ```
   */
  async setMetadata(metadata: Record<string, unknown>): Promise<void> {
    await host.put(this.url('metadata'), { metadata });
  }

  /**
   * Gets the world-space transform of the persona entity.
   *
   * @example
   * ```typescript
   * const t = await p.transform();
   * console.log(`Position: ${t.translation}`);
   * ```
   */
  async transform(): Promise<Transform> {
    const response = await host.get(this.url('transform'));
    return (await response.json()) as Transform;
  }

  /**
   * Sets the world-space transform of the persona entity.
   *
   * @param transform - The new transform (partial fields accepted by the server)
   *
   * @example
   * ```typescript
   * await p.setTransform({ translation: [0, 1, 0] });
   * ```
   */
  async setTransform(transform: Partial<Transform>): Promise<void> {
    await host.put(this.url('transform'), transform);
  }

  /**
   * Applies a partial update to the persona, only modifying specified fields.
   *
   * @param fields - The fields to update
   * @returns The updated persona snapshot
   *
   * @example
   * ```typescript
   * const updated = await p.patch({ name: "Alice v2", personality: "Bold and daring" });
   * console.log(updated.name);
   * ```
   */
  async patch(fields: PatchPersona): Promise<PersonaSnapshot> {
    const response = await host.patch(this.url(), fields);
    return (await response.json()) as PersonaSnapshot;
  }

  /**
   * Gets the current state of the persona.
   *
   * @example
   * ```typescript
   * const state = await p.state();
   * console.log(`Current state: ${state}`);
   * ```
   */
  async state(): Promise<string> {
    const response = await host.get(this.url('state'));
    const body = (await response.json()) as { state: string };
    return body.state;
  }

  /**
   * Sets the state of the persona.
   *
   * @param state - The new state string
   *
   * @example
   * ```typescript
   * await p.setState("talking");
   * ```
   */
  async setState(state: string): Promise<void> {
    await host.put(this.url('state'), { state });
  }

  /**
   * Spawns an ECS entity for this persona from the database record.
   * Does not attach VRM — call {@link attachVrm} after spawning.
   *
   * @returns The persona snapshot with `spawned: true`
   * @throws {HomunculusApiError} 409 if already spawned, 404 if persona not found
   *
   * @example
   * ```typescript
   * const snapshot = await p.spawn();
   * if (snapshot.vrmAssetId) {
   *   await p.attachVrm(snapshot.vrmAssetId);
   * }
   * ```
   */
  async spawn(): Promise<PersonaSnapshot> {
    const response = await host.post(this.url('spawn'), {});
    return (await response.json()) as PersonaSnapshot;
  }

  /**
   * Despawns the ECS entity for this persona, retaining the database record.
   *
   * @throws {HomunculusApiError} 404 if not spawned
   *
   * @example
   * ```typescript
   * await p.despawn();
   * ```
   */
  async despawn(): Promise<void> {
    await host.post(this.url('despawn'), {});
  }

  /**
   * Attaches a VRM model to this persona.
   *
   * @param assetId - The asset ID of the VRM model (e.g. "elmer:model")
   * @returns A {@link PersonaVrm} accessor for the attached VRM
   *
   * @example
   * ```typescript
   * const vrm = await p.attachVrm("elmer:model");
   * await vrm.playVrma({ asset: "vrma:idle-maid", repeat: { type: "forever" } });
   * ```
   */
  async attachVrm(assetId: string): Promise<PersonaVrm> {
    await host.post(this.url('vrm'), { assetId });
    return new PersonaVrm(this.id);
  }

  /**
   * Detaches the VRM model from this persona.
   *
   * @example
   * ```typescript
   * await p.detachVrm();
   * ```
   */
  async detachVrm(): Promise<void> {
    await host.deleteMethod(this.url('vrm'));
  }

  /**
   * Gets the thumbnail asset ID of the persona.
   *
   * @returns The asset ID, or `null` if no thumbnail is set.
   *
   * @example
   * ```typescript
   * const assetId = await p.thumbnail();
   * ```
   */
  async thumbnail(): Promise<string | null> {
    const response = await host.get(this.url('thumbnail'));
    const body = (await response.json()) as { thumbnail: string | null };
    return body.thumbnail;
  }

  /**
   * Sets the thumbnail asset ID of the persona.
   *
   * @param assetId - The asset ID to use as the thumbnail.
   * @returns The updated persona snapshot.
   *
   * @example
   * ```typescript
   * await p.setThumbnail("image:my-thumb");
   * ```
   */
  async setThumbnail(assetId: string): Promise<PersonaSnapshot> {
    const response = await host.put(this.url('thumbnail'), { thumbnail: assetId });
    return (await response.json()) as PersonaSnapshot;
  }

  /**
   * Clears the thumbnail of the persona.
   *
   * @returns The updated persona snapshot.
   *
   * @example
   * ```typescript
   * await p.clearThumbnail();
   * ```
   */
  async clearThumbnail(): Promise<PersonaSnapshot> {
    const response = await host.put(this.url('thumbnail'), { thumbnail: null });
    return (await response.json()) as PersonaSnapshot;
  }

  /**
   * Returns the URL for this persona's thumbnail image.
   *
   * Constructs a URL pointing to `/assets/file?id={thumbnail}` for the
   * given asset ID. Returns `null` if no thumbnail is set.
   *
   * @param thumbnail - The thumbnail asset ID (from `PersonaSnapshot.thumbnail`)
   * @returns The thumbnail image URL, or `null` if no thumbnail is set
   *
   * @example
   * ```typescript
   * const snap = await p.snapshot();
   * const url = p.thumbnailUrl(snap.thumbnail);
   * if (url) {
   *   // Use in React: <img src={url} alt={snap.name} />
   * }
   * ```
   */
  thumbnailUrl(thumbnail?: string | null): string | null {
    if (!thumbnail) return null;
    return host.createUrl('assets/file', { id: thumbnail }).toString();
  }

  /**
   * Returns a {@link PersonaVrm} accessor for operating on the attached VRM.
   *
   * This does not check whether a VRM is actually attached; methods on the
   * returned object will throw if no VRM is present.
   *
   * @example
   * ```typescript
   * const vrm = p.vrm();
   * await vrm.setExpressions({ happy: 1.0 });
   * ```
   */
  vrm(): PersonaVrm {
    return new PersonaVrm(this.id);
  }

  /**
   * Returns an SSE event source for receiving events related to this persona.
   *
   * @example
   * ```typescript
   * const events = p.events();
   * events.on("state-change", (data) => {
   *   console.log("New state:", data.state);
   * });
   * ```
   */
  events(): PersonaEventSource {
    const url = host.createUrl(`personas/${encodeURIComponent(this.id)}/events`);
    return new PersonaEventSource(new EventSource(url));
  }

  /**
   * Deletes this persona from the server.
   *
   * @example
   * ```typescript
   * await p.delete();
   * ```
   */
  async delete(): Promise<void> {
    await host.deleteMethod(this.url());
  }

  // --- Static factory methods ---

  /**
   * Creates a new persona and returns a {@link Persona} instance.
   *
   * @param params - Creation parameters including ID and optional fields
   * @returns A new Persona instance
   *
   * @example
   * ```typescript
   * const p = await Persona.create({
   *   id: "alice",
   *   name: "Alice",
   *   profile: "A cheerful virtual assistant",
   *   personality: "Friendly and curious",
   *   gender: "female",
   * });
   * ```
   */
  static async create(params: {
    id: string;
    name?: string;
    profile?: string;
    personality?: string;
    /** Optional VRM asset ID to attach at creation time. */
    vrmAssetId?: string;
    /** Optional thumbnail asset ID to set at creation time. */
    thumbnail?: string;
    gender?: Gender;
    age?: number;
    firstPersonPronoun?: string;
    metadata?: Record<string, unknown>;
  }): Promise<Persona> {
    const response = await host.post(host.createUrl('personas'), params);
    const snapshot = (await response.json()) as PersonaSnapshot;
    return new Persona(snapshot.id);
  }

  /**
   * Loads an existing persona by ID.
   *
   * @param id - The persona ID to load
   * @returns A Persona instance
   * @throws {HomunculusApiError} If the persona does not exist (404)
   *
   * @example
   * ```typescript
   * const p = await Persona.load("alice");
   * const name = await p.name();
   * ```
   */
  static async load(id: string): Promise<Persona> {
    // Verify it exists by fetching it
    await host.get(host.createUrl(`personas/${encodeURIComponent(id)}`));
    return new Persona(id);
  }

  /**
   * Lists all personas.
   *
   * @returns An array of persona snapshots
   *
   * @example
   * ```typescript
   * const all = await Persona.list();
   * for (const p of all) {
   *   console.log(`${p.id}: ${p.name ?? "(unnamed)"}`);
   * }
   * ```
   */
  static async list(): Promise<PersonaSnapshot[]> {
    const response = await host.get(host.createUrl('personas'));
    return (await response.json()) as PersonaSnapshot[];
  }
}

// --- VrmaRepeat & repeat helpers ---

/** Repeat settings for VRMA playback. */
export interface VrmaRepeat {
  type: 'forever' | 'never' | 'count';
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
    return { type: 'forever' };
  }

  /**
   * Play the animation once (no repeat).
   */
  export function never(): VrmaRepeat {
    return { type: 'never' };
  }

  /**
   * Repeat the animation a fixed number of times.
   *
   * @param n Positive integer repeat count.
   * @throws {RangeError} If `n` is not a positive integer.
   */
  export function count(n: number): VrmaRepeat {
    if (!Number.isInteger(n) || !Number.isFinite(n) || n <= 0) {
      throw new RangeError('repeat.count(n) requires a positive integer');
    }
    return { type: 'count', count: n };
  }
}

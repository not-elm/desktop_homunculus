/**
 * Character lifecycle management for Desktop Homunculus.
 *
 * A character represents a character identity that may or may not have a VRM model
 * attached. Characters are the primary abstraction for managing characters — they
 * persist across VRM attach/detach cycles and hold identity data (name, persona,
 * state) independently of the 3D model.
 *
 * @example
 * ```typescript
 * import { Character } from "@hmcs/sdk";
 *
 * // Create and attach a VRM model
 * const character = await Character.spawn("elmer", "vrm:elmer");
 * console.log(await character.name()); // "elmer"
 *
 * // Access the VRM model for animation/expression control
 * const vrm = character.vrm();
 * await vrm.playVrma({ asset: "vrma:idle-maid", repeat: repeat.forever() });
 * ```
 *
 * @packageDocumentation
 */

import { host } from "./host";
import { type Persona } from "./vrm";
import { Vrm } from "./vrm";

// --- Character types ---

/**
 * Options for creating a new character.
 *
 * If a character with the given ID already exists, it is returned without
 * creating a duplicate (upsert semantics).
 *
 * @example
 * ```typescript
 * const options: CreateCharacterOptions = {
 *   id: "elmer",
 *   assetId: "vrm:elmer",
 *   name: "Elmer",
 * };
 * ```
 */
export interface CreateCharacterOptions {
  /** Unique character identifier. */
  id: string;
  /** Optional display name. Defaults to the character ID if omitted. */
  name?: string;
}

/**
 * Options for spawning a character with a VRM model.
 *
 * @example
 * ```typescript
 * const options: SpawnCharacterOptions = {
 *   name: "Elmer",
 *   persona: { profile: "A cheerful assistant" },
 * };
 * ```
 */
export interface SpawnCharacterOptions {
  /** Optional display name. */
  name?: string;
  /** Optional persona to set on the character. */
  persona?: Partial<Persona>;
}

/**
 * Summary information about a character.
 *
 * @example
 * ```typescript
 * const characters = await Character.findAll();
 * for (const info of characters) {
 *   console.log(`${info.id}: ${info.name} (hasVrm=${info.hasVrm})`);
 * }
 * ```
 */
export interface CharacterInfo {
  /** Unique character identifier. */
  id: string;
  /** Display name. */
  name: string;
  /** Current character state (e.g. "idle", "sitting"). */
  state: string;
  /** Whether a VRM model is currently attached. */
  hasVrm: boolean;
  /** The underlying Bevy entity ID. */
  entity: number;
}

/**
 * Detailed character information including persona data.
 *
 * @example
 * ```typescript
 * const character = await Character.find("elmer");
 * const detail = await character.detail();
 * console.log(detail.persona.profile);
 * ```
 */
export interface CharacterDetail extends CharacterInfo {
  /** The character's persona. */
  persona: Persona;
}

// --- Character class ---

/**
 * Represents a character — a character identity that may or may not have a VRM model attached.
 *
 * Characters are the primary abstraction for managing characters in Desktop Homunculus.
 * They hold identity data (name, persona, state) independently of the 3D VRM model,
 * allowing characters to persist across model attach/detach cycles.
 *
 * @example
 * ```typescript
 * // Create a character and attach a VRM model
 * const character = await Character.spawn("elmer", "vrm:elmer");
 * console.log(await character.name()); // "elmer"
 *
 * // Control the attached VRM
 * const vrm = character.vrm();
 * await vrm.setExpressions({ happy: 1.0 });
 *
 * // Detach and reattach a different model
 * await character.detachVrm();
 * await character.attachVrm("vrm:another-model");
 * ```
 */
export class Character {
  /** The character's unique identifier. */
  readonly characterId: string;

  /** The underlying Bevy entity ID (internal use). */
  readonly entity: number;

  constructor(characterId: string, entity: number) {
    this.characterId = characterId;
    this.entity = entity;
  }

  // --- Static Methods ---

  /**
   * Creates a new character.
   *
   * @param options - Character creation options including ID, asset ID, and optional name.
   * @returns A new Character instance.
   *
   * @example
   * ```typescript
   * const character = await Character.create({ id: "elmer", assetId: "vrm:elmer" });
   * ```
   */
  static async create(options: CreateCharacterOptions): Promise<Character> {
    const url = host.createUrl("characters");
    const body = {
      id: options.id,
      name: options.name,
    };
    const response = await host.post(url, body);
    const result = (await response.json()) as CharacterInfo;
    return new Character(result.id, result.entity);
  }

  /**
   * Finds a character by its ID.
   *
   * @param id - The character's unique identifier.
   * @returns The Character instance.
   * @throws {HomunculusApiError} If no character exists with the given ID.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * console.log(await character.name());
   * ```
   */
  static async find(id: string): Promise<Character> {
    const response = await host.get(host.createUrl(`characters/${id}`));
    const info = (await response.json()) as CharacterDetail;
    return new Character(info.id, info.entity);
  }

  /**
   * Lists all registered characters.
   *
   * @returns An array of character summary information.
   *
   * @example
   * ```typescript
   * const characters = await Character.findAll();
   * for (const info of characters) {
   *   console.log(`${info.id}: ${info.name}`);
   * }
   * ```
   */
  static async findAll(): Promise<CharacterInfo[]> {
    const response = await host.get(host.createUrl("characters"));
    return (await response.json()) as CharacterInfo[];
  }

  // --- Instance Methods ---

  /**
   * Returns the character's display name.
   *
   * @returns The character's current display name.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * const name = await character.name();
   * console.log(name); // "Elmer"
   * ```
   */
  async name(): Promise<string> {
    const response = await host.get(
      host.createUrl(`characters/${this.characterId}/name`),
    );
    const result = (await response.json()) as { name: string };
    return result.name;
  }

  /**
   * Sets the character's display name.
   *
   * @param name - The new display name.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.setName("Elmer the Great");
   * ```
   */
  async setName(name: string): Promise<void> {
    await host.put(host.createUrl(`characters/${this.characterId}/name`), {
      name,
    });
  }

  /**
   * Returns the character's persona.
   *
   * @returns The character's persona data.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * const persona = await character.persona();
   * console.log(persona.profile);
   * ```
   */
  async persona(): Promise<Persona> {
    const response = await host.get(
      host.createUrl(`characters/${this.characterId}/persona`),
    );
    return (await response.json()) as Persona;
  }

  /**
   * Sets the character's persona.
   *
   * @param persona - Partial persona data to set. Only provided fields are updated.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.setPersona({
   *   profile: "A cheerful virtual assistant",
   *   ocean: { openness: 0.8, extraversion: 0.7 },
   * });
   * ```
   */
  async setPersona(persona: Partial<Persona>): Promise<void> {
    await host.put(
      host.createUrl(`characters/${this.characterId}/persona`),
      persona,
    );
  }

  /**
   * Returns the character's current state (e.g., "idle", "sitting").
   *
   * @returns The current state string.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * const state = await character.state();
   * console.log(state); // "idle"
   * ```
   */
  async state(): Promise<string> {
    const response = await host.get(
      host.createUrl(`characters/${this.characterId}/state`),
    );
    const result = (await response.json()) as { state: string };
    return result.state;
  }

  /**
   * Sets the character's state.
   *
   * @param state - The new state string (e.g. "idle", "sitting").
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.setState("sitting");
   * ```
   */
  async setState(state: string): Promise<void> {
    await host.put(host.createUrl(`characters/${this.characterId}/state`), {
      state,
    });
  }

  /**
   * Returns a {@link Vrm} instance for controlling the attached VRM model.
   *
   * The returned Vrm uses the existing `/vrm/{entity}/...` routes for model
   * control (expressions, animations, spring bones, etc.).
   *
   * @returns A Vrm instance bound to this character's entity.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * const vrm = character.vrm();
   * await vrm.setExpressions({ happy: 1.0 });
   * await vrm.playVrma({ asset: "vrma:idle-maid" });
   * ```
   */
  vrm(): Vrm {
    console.log("+++++++", this.entity);
    return new Vrm(this.entity, this.characterId);
  }

  /**
   * Attaches a VRM model to this character.
   *
   * @param assetId - Asset ID of the VRM model to attach (e.g. "vrm:elmer").
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.attachVrm("vrm:elmer");
   * ```
   */
  async attachVrm(assetId: string): Promise<Vrm> {
    await host.post(
      host.createUrl(`characters/${this.characterId}/vrm/attach`),
      { assetId },
    );
    return this.vrm();
  }

  /**
   * Detaches the VRM model from this character.
   *
   * The character continues to exist without a 3D model. A new model can be
   * attached later via {@link Character.attachVrm}.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.detachVrm();
   * ```
   */
  async detachVrm(): Promise<void> {
    await host.deleteMethod(
      host.createUrl(`characters/${this.characterId}/vrm`),
    );
  }

  /**
   * Destroys this character and its attached VRM (if any).
   *
   * After calling this method, the character instance should not be used.
   *
   * @example
   * ```typescript
   * const character = await Character.find("elmer");
   * await character.destroy();
   * ```
   */
  async destroy(): Promise<void> {
    await host.deleteMethod(host.createUrl(`characters/${this.characterId}`));
  }
}

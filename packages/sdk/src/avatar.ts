/**
 * Avatar lifecycle management for Desktop Homunculus.
 *
 * An avatar represents a character identity that may or may not have a VRM model
 * attached. Avatars are the primary abstraction for managing characters — they
 * persist across VRM attach/detach cycles and hold identity data (name, persona,
 * state) independently of the 3D model.
 *
 * @example
 * ```typescript
 * import { Avatar } from "@hmcs/sdk";
 *
 * // Create and attach a VRM model
 * const avatar = await Avatar.spawn("elmer", "vrm:elmer");
 * console.log(await avatar.name()); // "elmer"
 *
 * // Access the VRM model for animation/expression control
 * const vrm = avatar.vrm();
 * await vrm.playVrma({ asset: "vrma:idle-maid", repeat: repeat.forever() });
 * ```
 *
 * @packageDocumentation
 */

import { host } from "./host";
import { type Persona } from "./vrm";
import { Vrm } from "./vrm";

// --- Avatar types ---

/**
 * Options for creating a new avatar.
 *
 * @example
 * ```typescript
 * const options: CreateAvatarOptions = {
 *   id: "elmer",
 *   assetId: "vrm:elmer",
 *   name: "Elmer",
 *   ensure: true,
 * };
 * ```
 */
export interface CreateAvatarOptions {
    /** Unique avatar identifier. */
    id: string;
    /** Asset ID of the VRM model (e.g. "vrm:elmer"). */
    assetId: string;
    /** Optional display name. Defaults to the avatar ID if omitted. */
    name?: string;
    /** If true, returns an existing avatar instead of failing when one exists with the same ID. */
    ensure?: boolean;
}

/**
 * Options for spawning an avatar with a VRM model.
 *
 * @example
 * ```typescript
 * const options: SpawnAvatarOptions = {
 *   name: "Elmer",
 *   persona: { profile: "A cheerful assistant" },
 * };
 * ```
 */
export interface SpawnAvatarOptions {
    /** Optional display name. */
    name?: string;
    /** Optional persona to set on the avatar. */
    persona?: Partial<Persona>;
}

/**
 * Summary information about an avatar.
 *
 * @example
 * ```typescript
 * const avatars = await Avatar.findAll();
 * for (const info of avatars) {
 *   console.log(`${info.id}: ${info.name} (hasVrm=${info.hasVrm})`);
 * }
 * ```
 */
export interface AvatarInfo {
    /** Unique avatar identifier. */
    id: string;
    /** Display name. */
    name: string;
    /** Asset ID of the associated VRM model. */
    assetId: string;
    /** Current avatar state (e.g. "idle", "sitting"). */
    state: string;
    /** Whether a VRM model is currently attached. */
    hasVrm: boolean;
    /** The underlying Bevy entity ID. */
    entity: number;
}

/**
 * Detailed avatar information including persona data.
 *
 * @example
 * ```typescript
 * const avatar = await Avatar.find("elmer");
 * const detail = await avatar.detail();
 * console.log(detail.persona.profile);
 * ```
 */
export interface AvatarDetail extends AvatarInfo {
    /** The avatar's persona. */
    persona: Persona;
}

// --- Avatar class ---

/**
 * Represents an avatar — a character identity that may or may not have a VRM model attached.
 *
 * Avatars are the primary abstraction for managing characters in Desktop Homunculus.
 * They hold identity data (name, persona, state) independently of the 3D VRM model,
 * allowing characters to persist across model attach/detach cycles.
 *
 * @example
 * ```typescript
 * // Create an avatar and attach a VRM model
 * const avatar = await Avatar.spawn("elmer", "vrm:elmer");
 * console.log(await avatar.name()); // "elmer"
 *
 * // Control the attached VRM
 * const vrm = avatar.vrm();
 * await vrm.setExpressions({ happy: 1.0 });
 *
 * // Detach and reattach a different model
 * await avatar.detachVrm();
 * await avatar.attachVrm("vrm:another-model");
 * ```
 */
export class Avatar {
    /** The avatar's unique identifier. */
    readonly avatarId: string;

    /** The underlying Bevy entity ID (internal use). */
    readonly entity: number;

    constructor(avatarId: string, entity: number) {
        this.avatarId = avatarId;
        this.entity = entity;
    }

    // --- Static Methods ---

    /**
     * Creates a new avatar.
     *
     * @param options - Avatar creation options including ID, asset ID, and optional name.
     * @returns A new Avatar instance.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.create({ id: "elmer", assetId: "vrm:elmer" });
     * ```
     */
    static async create(options: CreateAvatarOptions): Promise<Avatar> {
        const params = options.ensure ? { ensure: "true" } : {};
        const url = host.createUrl("avatars", params);
        const body = { id: options.id, assetId: options.assetId, name: options.name };
        const response = await host.post(url, body);
        const result = await response.json() as AvatarInfo;
        return new Avatar(result.id, result.entity);
    }

    /**
     * Creates an avatar and immediately attaches a VRM model.
     *
     * This is a convenience method that combines {@link Avatar.create} with
     * {@link Avatar.attachVrm} and optional persona configuration.
     *
     * @param id - Unique avatar identifier.
     * @param assetId - Asset ID of the VRM model to attach (e.g. "vrm:elmer").
     * @param options - Optional spawn settings (name, persona).
     * @returns A new Avatar instance with a VRM model attached.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.spawn("elmer", "vrm:elmer");
     * ```
     */
    static async spawn(id: string, assetId: string, options?: SpawnAvatarOptions): Promise<Avatar> {
        const avatar = await Avatar.create({
            id,
            assetId,
            name: options?.name,
            ensure: true,
        });
        await avatar.attachVrm(assetId);
        if (options?.persona) {
            await avatar.setPersona(options.persona);
        }
        return avatar;
    }

    /**
     * Finds an avatar by its ID.
     *
     * @param id - The avatar's unique identifier.
     * @returns The Avatar instance.
     * @throws {HomunculusApiError} If no avatar exists with the given ID.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * console.log(await avatar.name());
     * ```
     */
    static async find(id: string): Promise<Avatar> {
        const response = await host.get(host.createUrl(`avatars/${id}`));
        const info = await response.json() as AvatarDetail;
        return new Avatar(info.id, info.entity);
    }

    /**
     * Lists all registered avatars.
     *
     * @returns An array of avatar summary information.
     *
     * @example
     * ```typescript
     * const avatars = await Avatar.findAll();
     * for (const info of avatars) {
     *   console.log(`${info.id}: ${info.name}`);
     * }
     * ```
     */
    static async findAll(): Promise<AvatarInfo[]> {
        const response = await host.get(host.createUrl("avatars"));
        return await response.json() as AvatarInfo[];
    }

    // --- Instance Methods ---

    /**
     * Returns the avatar's display name.
     *
     * @returns The avatar's current display name.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * const name = await avatar.name();
     * console.log(name); // "Elmer"
     * ```
     */
    async name(): Promise<string> {
        const response = await host.get(host.createUrl(`avatars/${this.avatarId}/name`));
        const result = await response.json() as { name: string };
        return result.name;
    }

    /**
     * Sets the avatar's display name.
     *
     * @param name - The new display name.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.setName("Elmer the Great");
     * ```
     */
    async setName(name: string): Promise<void> {
        await host.put(host.createUrl(`avatars/${this.avatarId}/name`), { name });
    }

    /**
     * Returns the avatar's persona.
     *
     * @returns The avatar's persona data.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * const persona = await avatar.persona();
     * console.log(persona.profile);
     * ```
     */
    async persona(): Promise<Persona> {
        const response = await host.get(host.createUrl(`avatars/${this.avatarId}/persona`));
        return await response.json() as Persona;
    }

    /**
     * Sets the avatar's persona.
     *
     * @param persona - Partial persona data to set. Only provided fields are updated.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.setPersona({
     *   profile: "A cheerful virtual assistant",
     *   ocean: { openness: 0.8, extraversion: 0.7 },
     * });
     * ```
     */
    async setPersona(persona: Partial<Persona>): Promise<void> {
        await host.put(host.createUrl(`avatars/${this.avatarId}/persona`), persona);
    }

    /**
     * Returns the avatar's current state (e.g., "idle", "sitting").
     *
     * @returns The current state string.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * const state = await avatar.state();
     * console.log(state); // "idle"
     * ```
     */
    async state(): Promise<string> {
        const response = await host.get(host.createUrl(`avatars/${this.avatarId}/state`));
        const result = await response.json() as { state: string };
        return result.state;
    }

    /**
     * Sets the avatar's state.
     *
     * @param state - The new state string (e.g. "idle", "sitting").
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.setState("sitting");
     * ```
     */
    async setState(state: string): Promise<void> {
        await host.put(host.createUrl(`avatars/${this.avatarId}/state`), { state });
    }

    /**
     * Returns a {@link Vrm} instance for controlling the attached VRM model.
     *
     * The returned Vrm uses the existing `/vrm/{entity}/...` routes for model
     * control (expressions, animations, spring bones, etc.).
     *
     * @returns A Vrm instance bound to this avatar's entity.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * const vrm = avatar.vrm();
     * await vrm.setExpressions({ happy: 1.0 });
     * await vrm.playVrma({ asset: "vrma:idle-maid" });
     * ```
     */
    vrm(): Vrm {
        return new Vrm(this.entity, this.avatarId);
    }

    /**
     * Attaches a VRM model to this avatar.
     *
     * @param assetId - Asset ID of the VRM model to attach (e.g. "vrm:elmer").
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.attachVrm("vrm:elmer");
     * ```
     */
    async attachVrm(assetId: string): Promise<void> {
        await host.post(host.createUrl(`avatars/${this.avatarId}/vrm/attach`), { assetId });
    }

    /**
     * Detaches the VRM model from this avatar.
     *
     * The avatar continues to exist without a 3D model. A new model can be
     * attached later via {@link Avatar.attachVrm}.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.detachVrm();
     * ```
     */
    async detachVrm(): Promise<void> {
        await host.deleteMethod(host.createUrl(`avatars/${this.avatarId}/vrm`));
    }

    /**
     * Destroys this avatar and its attached VRM (if any).
     *
     * After calling this method, the avatar instance should not be used.
     *
     * @example
     * ```typescript
     * const avatar = await Avatar.find("elmer");
     * await avatar.destroy();
     * ```
     */
    async destroy(): Promise<void> {
        await host.deleteMethod(host.createUrl(`avatars/${this.avatarId}`));
    }
}

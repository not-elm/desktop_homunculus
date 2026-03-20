/**
 * Avatar extension data API for per-mod key-value storage on avatars.
 *
 * Extensions allow mods to attach arbitrary JSON data to an avatar,
 * persisted in the avatar database. Each mod gets its own isolated
 * namespace keyed by `modName`.
 *
 * @packageDocumentation
 */

import { host } from "./host";

/**
 * Gets extension data stored for a mod on an avatar.
 *
 * @param avatarId - The avatar identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @returns The stored extension data
 * @throws {HomunculusApiError} If the avatar or extension data is not found
 *
 * @example
 * ```ts
 * const data = await getExtension<MySettings>("elmer", "@hmcs/voicevox");
 * ```
 */
export async function getExtension<T = unknown>(avatarId: string, modName: string): Promise<T> {
    const url = extensionUrl(avatarId, modName);
    const response = await host.get(url);
    return response.json() as Promise<T>;
}

/**
 * Sets extension data for a mod on an avatar.
 *
 * Creates or replaces any existing data for the given mod.
 *
 * @param avatarId - The avatar identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @param data - The data to store (will be JSON-serialized)
 * @throws {HomunculusApiError} If the avatar is not found
 *
 * @example
 * ```ts
 * await setExtension("elmer", "@hmcs/voicevox", { speed: 1.2 });
 * ```
 */
export async function setExtension(avatarId: string, modName: string, data: unknown): Promise<void> {
    const url = extensionUrl(avatarId, modName);
    await host.put(url, data);
}

/**
 * Deletes extension data for a mod on an avatar.
 *
 * @param avatarId - The avatar identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @throws {HomunculusApiError} If the avatar is not found
 *
 * @example
 * ```ts
 * await deleteExtension("elmer", "@hmcs/voicevox");
 * ```
 */
export async function deleteExtension(avatarId: string, modName: string): Promise<void> {
    const url = extensionUrl(avatarId, modName);
    await host.deleteMethod(url);
}

/**
 * Normalizes a mod name for use in URL paths.
 *
 * Replaces `@` and `/` with URL-safe characters so that scoped
 * npm package names can be used as path segments.
 *
 * @param modName - The original mod name
 * @returns The normalized name suitable for URL paths
 *
 * @example
 * ```ts
 * normalizeModName("@hmcs/voicevox") // "hmcs__voicevox"
 * normalizeModName("my-mod")         // "my-mod"
 * ```
 */
export function normalizeModName(modName: string): string {
    return modName.replace(/^@/, "").replace(/\//g, "__");
}

/** Builds the URL for an extension endpoint. */
function extensionUrl(avatarId: string, modName: string): URL {
    const normalized = normalizeModName(modName);
    return host.createUrl(`avatars/${avatarId}/extensions/${normalized}`);
}

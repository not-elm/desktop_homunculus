/**
 * Character extension data API for per-mod key-value storage on characters.
 *
 * Extensions allow mods to attach arbitrary JSON data to a character,
 * persisted in the character database. Each mod gets its own isolated
 * namespace keyed by `modName`.
 *
 * @packageDocumentation
 */

import { host } from "./host";

/**
 * Gets extension data stored for a mod on a character.
 *
 * @param characterId - The character identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @returns The stored extension data
 * @throws {HomunculusApiError} If the character or extension data is not found
 *
 * @example
 * ```ts
 * const data = await getExtension<MySettings>("elmer", "@hmcs/voicevox");
 * ```
 */
export async function getExtension<T = unknown>(characterId: string, modName: string): Promise<T> {
    const url = extensionUrl(characterId, modName);
    const response = await host.get(url);
    return response.json() as Promise<T>;
}

/**
 * Sets extension data for a mod on a character.
 *
 * Creates or replaces any existing data for the given mod.
 *
 * @param characterId - The character identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @param data - The data to store (will be JSON-serialized)
 * @throws {HomunculusApiError} If the character is not found
 *
 * @example
 * ```ts
 * await setExtension("elmer", "@hmcs/voicevox", { speed: 1.2 });
 * ```
 */
export async function setExtension(characterId: string, modName: string, data: unknown): Promise<void> {
    const url = extensionUrl(characterId, modName);
    await host.put(url, data);
}

/**
 * Deletes extension data for a mod on a character.
 *
 * @param characterId - The character identifier
 * @param modName - The mod name (e.g. `"@hmcs/voicevox"`)
 * @throws {HomunculusApiError} If the character is not found
 *
 * @example
 * ```ts
 * await deleteExtension("elmer", "@hmcs/voicevox");
 * ```
 */
export async function deleteExtension(characterId: string, modName: string): Promise<void> {
    const url = extensionUrl(characterId, modName);
    await host.deleteMethod(url);
}

/** Builds the URL for an extension endpoint. */
function extensionUrl(characterId: string, modName: string): URL {
    return host.createUrl(`characters/${characterId}/extensions`, { mod: modName });
}

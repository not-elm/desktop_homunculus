import { HomunculusApiError, host } from './host';

/**
 * Preferences API namespace for persistent data storage and user settings.
 *
 * Provides a key-value store for saving and loading application data that persists
 * across sessions.
 *
 * @example
 * ```typescript
 * await preferences.save('user-settings', { theme: 'dark', volume: 0.8 });
 * const settings = await preferences.load<{ theme: string; volume: number }>('user-settings');
 * ```
 */
export namespace preferences {
  /**
   * List all saved preference keys.
   *
   * Returns an array of key names that have been stored.
   * Use {@link preferences.load} to retrieve the value for a specific key.
   *
   * @returns Array of preference key names
   *
   * @example
   * ```typescript
   * const keys = await preferences.list();
   * console.log(`${keys.length} preferences stored`);
   *
   * // Load all preferences
   * for (const key of keys) {
   *   const value = await preferences.load(key);
   *   console.log(`${key}:`, value);
   * }
   * ```
   */
  export async function list(): Promise<string[]> {
    const response = await host.get(host.createUrl('preferences'));
    return await response.json();
  }

  /**
   * Loads a value from the preference store with type safety.
   *
   * Returns `undefined` if the key does not exist.
   *
   * @template V - The expected type of the stored value
   * @param key - The unique identifier for the stored data
   * @returns A promise that resolves to the deserialized value, or `undefined` if the key does not exist
   *
   * @example
   * ```typescript
   * const username = await preferences.load<string>('username');
   * if (username !== null) {
   *   console.log(`Hello, ${username}`);
   * }
   * ```
   */
  export async function load<V>(key: string): Promise<V | undefined> {
    try {
      const response = await host.get(host.createUrl(`preferences/${key}`));
      return (await response.json()) as V;
    } catch (e) {
      if (e instanceof HomunculusApiError && e.statusCode === 404) {
        return undefined;
      }
      throw e;
    }
  }

  /**
   * Saves a value to the preference store with automatic serialization.
   *
   * @template V - The type of the value being saved
   * @param key - The unique identifier for storing the data
   * @param value - The data to save (must be JSON-serializable)
   *
   * @example
   * ```typescript
   * await preferences.save('username', 'Alice');
   * ```
   */
  export async function save<V>(key: string, value: V): Promise<void> {
    await host.put(host.createUrl(`preferences/${key}`), value);
  }
}

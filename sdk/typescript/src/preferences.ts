import {host} from "./host";
import {Transform} from "./math";

/**
 * Preferences API namespace for persistent data storage and user settings.
 * 
 * Provides a key-value store for saving and loading application data that persists
 * across sessions. This is ideal for user preferences, configuration data, VRM positions,
 * and any other data that should be remembered between application runs.
 * 
 * Key features:
 * - Generic type-safe data storage
 * - Automatic JSON serialization/deserialization
 * - VRM-specific transform persistence helpers
 * - Error handling for missing keys
 * - Cross-session data persistence
 * 
 * @example
 * ```typescript
 * // Save and load user preferences
 * interface UserSettings {
 *   theme: 'dark' | 'light';
 *   volume: number;
 *   autoSave: boolean;
 * }
 * 
 * const settings: UserSettings = {
 *   theme: 'dark',
 *   volume: 0.8,
 *   autoSave: true
 * };
 * 
 * await preferences.save('user-settings', settings);
 * const loadedSettings = await preferences.load<UserSettings>('user-settings');
 * 
 * // Save VRM positions
 * const vrm = await Vrm.findByName('MyCharacter');
 * const currentTransform = await entities.transform(vrm.entity);
 * await preferences.saveVrmTransform('MyCharacter', currentTransform);
 * 
 * // Restore VRM positions on startup
 * const savedTransform = await preferences.loadVrmTransform('MyCharacter');
 * await entities.setTransform(vrm.entity, savedTransform);
 * 
 * // Store complex application state
 * interface AppState {
 *   activeVrms: string[];
 *   openWebviews: number[];
 *   lastUsedMods: string[];
 * }
 * 
 * const appState: AppState = {
 *   activeVrms: ['Character1', 'Character2'],
 *   openWebviews: [123, 456],
 *   lastUsedMods: ['chat-mod', 'weather-mod']
 * };
 * 
 * await preferences.save('app-state', appState);
 * ```
 */
export namespace preferences {
    /**
     * Loads a value from the preference store with type safety.
     * 
     * Retrieves and deserializes data that was previously saved with the same key.
     * The data is automatically parsed from JSON format back to the original type.
     * 
     * @template V - The expected type of the stored value
     * @param key - The unique identifier for the stored data
     * @returns A promise that resolves to the deserialized value
     * @throws Will throw an error if the key does not exist or cannot be parsed
     * 
     * @example
     * ```typescript
     * // Load simple values
     * const username = await preferences.load<string>('username');
     * const volume = await preferences.load<number>('audio-volume');
     * 
     * // Load complex objects with type safety
     * interface GameSettings {
     *   difficulty: 'easy' | 'normal' | 'hard';
     *   graphics: { quality: number; shadows: boolean };
     * }
     * 
     * const settings = await preferences.load<GameSettings>('game-settings');
     * console.log(`Difficulty: ${settings.difficulty}`);
     * 
     * // Handle missing keys with try-catch
     * try {
     *   const data = await preferences.load<any>('optional-data');
     *   console.log('Data found:', data);
     * } catch (error) {
     *   console.log('No data found, using defaults');
     *   // Set default values...
     * }
     * 
     * // Load arrays and nested structures
     * const favoriteVrms = await preferences.load<string[]>('favorite-vrms');
     * const modConfigs = await preferences.load<Record<string, any>>('mod-configs');
     * ```
     */
    export const load = async <V>(key: string): Promise<V> => {
        const response = await host.get(host.createUrl(`preferences/${key}`));
        return await response.json() as V;
    }

    /**
     * Saves a value to the preference store with automatic serialization.
     * 
     * Stores data persistently using JSON serialization. The data will be available
     * across application restarts and can be retrieved using the same key.
     * If a value already exists for the given key, it will be overwritten.
     * 
     * @template V - The type of the value being saved
     * @param key - The unique identifier for storing the data
     * @param value - The data to save (must be JSON-serializable)
     * 
     * @example
     * ```typescript
     * // Save simple values
     * await preferences.save('username', 'Alice');
     * await preferences.save('last-login', new Date().toISOString());
     * await preferences.save('session-count', 42);
     * 
     * // Save complex objects
     * const userProfile = {
     *   name: 'Alice',
     *   preferences: {
     *     theme: 'dark',
     *     notifications: true,
     *     autoBackup: false
     *   },
     *   favoriteVrms: ['Character1', 'Character2']
     * };
     * 
     * await preferences.save('user-profile', userProfile);
     * 
     * // Save application state for persistence
     * const currentState = {
     *   openWindows: await getOpenWebviewIds(),
     *   activeVrms: (await Vrm.findAll()).map(vrm => vrm.entity),
     *   lastModUsed: 'weather-widget',
     *   timestamp: Date.now()
     * };
     * 
     * await preferences.save('app-state', currentState);
     * 
     * // Periodic autosave pattern
     * setInterval(async () => {
     *   const gameData = {
     *     score: currentScore,
     *     level: currentLevel,
     *     inventory: playerInventory
     *   };
     *   await preferences.save('autosave', gameData);
     * }, 30000); // Save every 30 seconds
     * ```
     */
    export const save = async <V>(key: string, value: V): Promise<void> => {
        await host.put(host.createUrl(`preferences/${key}`), value);
    }

    /**
     * Loads a previously saved VRM transform by character name.
     * 
     * This is a convenience function that retrieves the transform (position, rotation, scale)
     * for a specific VRM character. If no saved transform exists, returns a default
     * identity transform instead of throwing an error.
     * 
     * @param vrmName - The name of the VRM character
     * @returns A promise that resolves to the saved transform, or default identity transform
     * 
     * @example
     * ```typescript
     * // Restore character positions on app startup
     * const characterNames = ['Alice', 'Bob', 'Charlie'];
     * 
     * for (const name of characterNames) {
     *   try {
     *     const vrm = await Vrm.findByName(name);
     *     const savedTransform = await preferences.loadVrmTransform(name);
     *     await entities.setTransform(vrm.entity, savedTransform);
     *     console.log(`Restored position for ${name}`);
     *   } catch (error) {
     *     console.log(`Could not restore ${name}: ${error.message}`);
     *   }
     * }
     * 
     * // Check if character has a saved position
     * const transform = await preferences.loadVrmTransform('MyCharacter');
     * const isAtDefaultPosition = 
     *   transform.translation[0] === 0 && 
     *   transform.translation[1] === 0 && 
     *   transform.translation[2] === 0;
     * 
     * if (isAtDefaultPosition) {
     *   console.log('Character is at default position');
     * }
     * ```
     */
    export const loadVrmTransform = async (
        vrmName: string
    ): Promise<Transform> => {
        try {
            return await preferences.load(`vrm::${vrmName}::transform`);
        } catch (e) {
            return {
                translation: [0, 0, 0],
                rotation: [0, 0, 0, 1],
                scale: [1, 1, 1]
            }
        }
    }

    /**
     * Saves a VRM character's transform for later restoration.
     * 
     * This convenience function stores the position, rotation, and scale of a VRM character
     * so it can be restored in future sessions. Useful for remembering where users
     * positioned their characters.
     * 
     * @param vrmName - The name of the VRM character
     * @param transform - The transform data to save
     * 
     * @example
     * ```typescript
     * // Save character position when user moves them
     * const vrm = await Vrm.findByName('MyCharacter');
     * const currentTransform = await entities.transform(vrm.entity);
     * await preferences.saveVrmTransform('MyCharacter', currentTransform);
     * 
     * // Auto-save character positions periodically
     * setInterval(async () => {
     *   const allVrms = await Vrm.findAll();
     *   
     *   for (const vrm of allVrms) {
     *     const name = await vrm.name();
     *     const transform = await entities.transform(vrm.entity);
     *     await preferences.saveVrmTransform(name, transform);
     *   }
     *   
     *   console.log(`Saved positions for ${allVrms.length} characters`);
     * }, 60000); // Every minute
     * 
     * // Save position when character stops moving
     * vrm.events().on('drag-end', async () => {
     *   const name = await vrm.name();
     *   const transform = await entities.transform(vrm.entity);
     *   await preferences.saveVrmTransform(name, transform);
     *   console.log(`Saved new position for ${name}`);
     * });
     * ```
     */
    export const saveVrmTransform = async (
        vrmName: string,
        transform: Transform
    ): Promise<void> => {
        await preferences.save(`vrm::${vrmName}::transform`, transform);
    }
}
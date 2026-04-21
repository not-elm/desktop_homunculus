import type { PersonaSnapshot } from '@hmcs/sdk';

/**
 * Returns a fully-typed `PersonaSnapshot` with sensible defaults, merged with
 * any provided overrides. Use this in tests instead of `as any` casts.
 */
export function makePersonaSnapshot(overrides: Partial<PersonaSnapshot> = {}): PersonaSnapshot {
  return {
    id: 'alice',
    name: 'Alice',
    age: null,
    gender: 'unknown',
    firstPersonPronoun: null,
    profile: '',
    personality: null,
    state: 'idle',
    vrmAssetId: null,
    thumbnail: null,
    metadata: {},
    spawned: true,
    ...overrides,
  };
}

/**
 * Returns the value from a `Map` for the given `key`, or throws if absent.
 * Use this in tests instead of `map.get(key)!` non-null assertions.
 */
export function getRequired<K, V>(map: Map<K, V>, key: K): V {
  const value = map.get(key);
  if (value === undefined) {
    throw new Error(`getRequired: key ${String(key)} not found in map`);
  }
  return value;
}

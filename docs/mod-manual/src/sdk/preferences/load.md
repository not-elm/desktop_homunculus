# Load Preferences

Loads a value from the preference store with type safety and automatic JSON deserialization.

## Parameters

- `key` (string) - The unique identifier for the stored data

## Returns

`Promise<V>` - A promise that resolves to the deserialized value of the specified type

## Description

The `load()` function retrieves and deserializes data that was previously saved with the same key. The data is
automatically parsed from JSON format back to the original type. If the key does not exist or cannot be parsed, the
function will throw an error.

## Examples

### Basic Data Loading

```typescript
import {preferences} from '@homunculus/sdk';

// Load simple values
const username = await preferences.load<string>('username');
const volume = await preferences.load<number>('audio-volume');
const isFirstRun = await preferences.load<boolean>('first-run');

console.log(`Welcome back, ${username}!`);
```

## Related Functions

- [`save()`](./save.md) - Save data to the preferences store
- [`loadVrmTransform()`](./loadVrmTransform.md) - Load VRM character positions
- [`saveVrmTransform()`](./saveVrmTransform.md) - Save VRM character positions

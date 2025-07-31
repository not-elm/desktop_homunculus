# Save Preferences

Saves a value to the preference store with automatic JSON serialization for persistent storage.

The `save()` function stores data persistently using JSON serialization. The data will be available across application
restarts and can be retrieved using the same key with `load()`. If a value already exists for the given key, it will be
overwritten.

## Parameters

- `key` (string) - The unique identifier for storing the data
- `value` (V) - The data to save (must be JSON-serializable)

## Returns

`Promise<void>` - A promise that resolves when the data has been saved

## Examples

### Basic Data Saving

```typescript
import {preferences} from '@homunculus/sdk';

// Save simple values
await preferences.save('username', 'Alice');
await preferences.save('last-login', new Date().toISOString());
await preferences.save('session-count', 42);
await preferences.save('tutorial-completed', true);

console.log('User data saved');
```

## Related Functions

- [`load()`](./load.md) - Load stored preferences with type safety
- [`loadVrmTransform()`](./loadVrmTransform.md) - Load VRM character positions
- [`saveVrmTransform()`](./saveVrmTransform.md) - Save VRM character positions

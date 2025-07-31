# Preferences

The Preferences API provides persistent data storage for mods, allowing you to save and load configuration data, user
settings, and application state that persists across sessions. This is ideal for storing user preferences, character
positions, mod configurations, and any other data that should be remembered between application runs.

## Key Features

- **Type-safe Storage**: Generic functions with TypeScript type safety
- **JSON Serialization**: Automatic serialization and deserialization of data
- **VRM Transform Helpers**: Specialized functions for saving character positions
- **Cross-session Persistence**: Data survives application restarts
- **Error Handling**: Graceful handling of missing keys and parse errors

## Quick Example

```typescript
import {preferences} from '@homunculus/sdk';

// Save user settings
interface UserSettings {
    theme: 'dark' | 'light';
    volume: number;
    autoSave: boolean;
}

const settings: UserSettings = {
    theme: 'dark',
    volume: 0.8,
    autoSave: true
};

await preferences.save('user-settings', settings);

// Load settings with type safety
const loadedSettings = await preferences.load<UserSettings>('user-settings');
console.log(`Theme: ${loadedSettings.theme}`);

// Save and restore character positions
const vrm = await Vrm.findByName('MyCharacter');
const transform = await entities.transform(vrm.entity);
await preferences.saveVrmTransform('MyCharacter', transform);

// Later...
const savedTransform = await preferences.loadVrmTransform('MyCharacter');
await entities.setTransform(vrm.entity, savedTransform);
```

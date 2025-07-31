# Retrieve MOD Menus

Retrieves all registered MOD menus metadata.

## Parameters

None

## Returns

`Promise<ModMenuMetadata>` - A promise that resolves when the data has been saved

## ModMenuMetadata

### thumbnail

Optional path to a thumbnail image for the mod.
The path should be relative to the mod's asset directory.

### text

Display name for the mod that appears in the menu.
This is the human-readable title users will see.

### script

The script local path relative to the mod's asset directory.

### webview

Optional webview configuration for how the mod should be displayed.
If not specified, default webview settings will be used.

## Examples

### Basic Data Saving

```typescript
import {mods} from '@homunculus/sdk';

const modMenus = await mods.menus();
```

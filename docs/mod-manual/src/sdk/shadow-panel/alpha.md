# Alpha Control

## shadowPanel.alpha

Fetches the current shadow panel's alpha value.(0-1)

### Parameters

None

### Returns

`Promise<number>` - A promise that resolves to the current alpha value of the shadow panel.

## shadowPanel.setAlpha

Sets the shadow panel's alpha value.(0-1)

The value is saved to storage and will persist across application restarts.

### Parameters

- `alpha` (number) - The new alpha value to set for the shadow panel. Must be between 0 and 1.

### Returns

`Promise<void>` - A promise that resolves when the alpha value has been successfully set.

### Examples

```typescript
import {shadowPanel} from '@homunculus/sdk';

// Set shadow panel to 50% opacity
await shadowPanel.setAlpha(0.5);
// Fetch the current alpha value
console.log(await shadowPanel.alpha());
```
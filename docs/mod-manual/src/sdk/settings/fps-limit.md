# Frame Rate Limit

The frame rate limit is a crucial performance setting that determines the maximum number of frames the application will
render per second. This setting helps optimize performance for different hardware configurations and can be used to
extend battery life on mobile devices or match display refresh rates.

## settings.fpsLimit()

Gets the current frame rate limit setting for the application.

The FPS limit controls how many frames per second the application will render, which directly affects performance, power
consumption, and visual smoothness. This function returns the currently configured maximum frame rate.

### Parameters

None.

### Returns

`Promise<number>` - The current FPS limit setting

## settings.saveFpsLimit(fps: number)

Sets the frame rate limit for the application with immediate effect.

This setting controls the maximum number of frames rendered per second, directly affecting performance, power
consumption, and visual smoothness. Changes take effect immediately without requiring an application restart.

### Parameters

- `fps`: The maximum frames per second to target

### Returns

`Promise<void>` - Resolves when the setting has been applied

## Examples

### Basic FPS Checking

```typescript
// Check current setting
const fps = await settings.fpsLimit();
console.log(`Current FPS limit: ${fps}`);

// Use in conditional logic
if (fps < 60) {
    console.log("Performance mode is enabled");
} else {
    console.log("High quality mode is enabled");
}
```

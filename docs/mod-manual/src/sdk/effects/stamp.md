# Stamp Effects

Displays a visual stamp effect on the screen using an image from mod assets.

Stamp effects are temporary visual elements that appear on screen for a short time.
They can be used for reactions, notifications, celebrations, or visual feedback. The image will appear at a random
position within the specified bounds.

## Parameters

- `source`: The mod image asset path relative to the `assets/mods` directory (e.g., `reactions/thumbs-up.png`)
- `options`: Optional configuration for the stamp appearance and behavior

### StampOptions

- `display?`: Display/monitor ID to show the effect on
- `bounds?`: Rectangular area where the stamp can appear (random position within bounds)
- `size?`: Size of the stamp effect in pixels [width, height] (default: [300, 300])
- `durationSecs?`: How long the stamp remains visible in seconds (default: 0.8)

## Examples

### Simple Stamp Effects

```typescript
// Simple stamp with default settings
await effects.stamp("reactions/thumbs-up.png");

// Customized stamp effect
await effects.stamp("celebrations/party.gif", {
    size: [150, 150],
    durationSecs: 2.5,
    bounds: {
        min: [200, 100],  // Top-left of allowed area
        max: [800, 500]   // Bottom-right of allowed area
    }
});
```

## Related Functions

- [`sound()`](./sound.md) - Play audio effects alongside visual stamps
- [`displays.findAll()`](../displays/findAll.md) - Target specific monitors for effects
- [`cameras.worldToGlobalViewport()`](../cameras/worldToGlobalViewport.md) - Position effects relative to 3D objects

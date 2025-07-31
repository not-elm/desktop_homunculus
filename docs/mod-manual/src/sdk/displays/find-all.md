# Find All Displays

Retrieves information about all currently connected displays/monitors.

This function queries the system to get real-time information about all available displays,
including their positions, sizes, and identifiers. The returned array includes both primary and secondary displays.

## Returns

Promise that resolves to an array of `Display` objects, each containing:

- `id`: Unique identifier for the display
- `title`: Human-readable name/title of the display
- `frame`: Rectangular bounds with `min` and `max` coordinates

## Examples

### Basic Display Enumeration

```typescript
// Get all displays
const allDisplays = await displays.findAll();
console.log(`System has ${allDisplays.length} displays`);

// Display detailed information
allDisplays.forEach((display, index) => {
    const width = display.frame.max[0] - display.frame.min[0];
    const height = display.frame.max[1] - display.frame.min[1];

    console.log(`Display ${index + 1}: ${display.title}`);
    console.log(`  Resolution: ${width}x${height}`);
    console.log(`  Position: (${display.frame.min[0]}, ${display.frame.min[1]})`);
});
```

## Related Functions

- [`effects.stamp()`](../effects/stamp.md) - Display effects on specific monitors
- [`webviews.open()`](../webviews/open.md) - Position webviews on specific displays
- [`cameras.worldToGlobalViewport()`](../cameras/worldToGlobalViewport.md) - Convert world coordinates to screen
  positions

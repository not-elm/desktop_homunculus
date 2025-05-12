# Displays API

The Displays API provides functionality for monitor and screen management. It allows you to query information about connected displays/monitors, including their dimensions, positions, and identifiers. This is essential for multi-monitor setups where you need to target specific screens for effects, webviews, or VRM positioning.

## Key Features

- **Discovery of Connected Displays**: Find all available displays/monitors
- **Display Identification**: Get unique identifiers and metadata for each display
- **Screen Bounds Information**: Access position and dimension data
- **Multi-monitor Support**: Target specific screens for various operations

## Functions

- [`findAll()`](./findAll.md) - Get information about all connected displays

## Quick Example

```typescript
// Get information about all connected displays
const allDisplays = await displays.findAll();

console.log(`Found ${allDisplays.length} displays:`);
allDisplays.forEach((display, index) => {
  console.log(`Display ${index + 1}:`);
  console.log(`  ID: ${display.id}`);
  console.log(`  Title: ${display.title}`);
  console.log(`  Size: ${display.frame.max[0]}x${display.frame.max[1]}`);
  console.log(`  Position: (${display.frame.min[0]}, ${display.frame.min[1]})`);
});

// Target a specific display for effects
const primaryDisplay = allDisplays[0];
await effects.stamp("celebration/confetti.gif", {
  display: primaryDisplay.id,
  bounds: primaryDisplay.frame
});

// Open webview on secondary monitor
if (allDisplays.length > 1) {
  const secondaryDisplay = allDisplays[1];
  await Webview.open("dashboard", {
    position: [
      secondaryDisplay.frame.min[0] + 100,
      secondaryDisplay.frame.min[1] + 100
    ]
  });
}
```

## Common Use Cases

- **Multi-monitor Applications**: Position content on specific monitors
- **Display Awareness**: Adapt UI based on available screen real estate
- **Effect Targeting**: Show effects on specific displays
- **Window Management**: Position webviews and UI elements precisely

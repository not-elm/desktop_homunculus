# Effects API

The Effects API provides functionality for playing visual and audio effects. It allows you to trigger various effects
that enhance the user experience, including sound effects and visual stamp effects that can be displayed on any monitor.

## Key Features

- **Sound Effect Playback**: Play audio effects from mod assets
- **Visual Stamp Effects**: Display temporary visual elements with customizable positioning and timing
- **Multi-monitor Support**: Target specific displays for effect placement
- **Asset-based System**: All effects reference mod assets for their content

## Functions

- [`sound()`](./sound.md) - Play audio effects from mod assets
- [`stamp()`](./stamp.md) - Display visual stamp effects with customizable options

## Quick Example

```typescript
// Play a sound effect
await effects.sound("notification-sounds/ding.wav");

// Show a stamp effect at a random position
await effects.stamp("reaction-images/heart.png", {
    size: [100, 100],
    durationSecs: 2.0
});

// Show stamp effect on a specific display with bounds
const displays = await displays.findAll();
await effects.stamp("celebrations/confetti.gif", {
    display: displays[1].id,  // Second monitor
    bounds: {
        min: [100, 100],
        max: [500, 400]
    },
    size: [200, 200],
    durationSecs: 3.0
});
```

## Common Use Cases

- **User Feedback**: Provide audio and visual feedback for user interactions
- **Notifications**: Alert users with sound and visual cues
- **Celebrations**: Show congratulatory effects for achievements
- **Ambient Effects**: Create atmospheric visual and audio experiences
- **Interactive Responses**: React to VRM interactions with appropriate effects

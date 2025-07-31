# Sound Effects

Plays a sound effect from a mod asset.

Sound effects are played immediately and do not block execution.
The sound file must be included in a mod's assets directory.
Supports common audio formats like WAV, MP3, and OGG.

## Parameters

- `source`: The mod asset path relative to the `assets/mods` directory.

## Examples

### Basic Sound Effects

```typescript
// Play notification sounds
await effects.sound("ui-sounds/button-click.wav");
await effects.sound("notifications/message-received.mp3");
await effects.sound("character-voices/greeting.ogg");
```

## Related Functions

- [`stamp()`](./stamp.md) - Display visual effects alongside sounds

# Vrm.speakOnVoiceVox()

Converts text to speech using the VoiceVox text-to-speech engine and makes the VRM character speak the provided text
with optional voice customization and subtitle display.

To use this method, [`VoiceVox`](https://voicevox.hiroshiba.jp/) must be running beforehand.
If `VoiceVox` is not running, the method will silently fail without throwing an error.

## Parameters

- `text`(string) - The text content to convert to speech
- `options`(SpeakOnVoiceVoxOptions, optional) - Configuration options for voice and subtitles

### SpeakOnVoiceVoxOptions

```typescript
interface SpeakOnVoiceVoxOptions {
    speaker?: number; // VoiceVox speaker ID (voice selection)
    subtitle?: SubtitleOptions; // Subtitle display configuration
}

```

### SubtitleOptions

If this option is specified, subtitles will be displayed at the bottom of the screen.

```typescript
interface SubtitleOptions {
    font?: string;                              // Mod asset ID for subtitle font
    fontSize?: number;                          // Font size for subtitles
    color?: [number, number, number, number];   // RGBA color values (0-1 range)
}
```

## Returns

`Promise<ReadableStream<string>>` - A readable stream of the speech synthesis response

## Description

The `speakOnVoiceVox()` method uses the VoiceVox text-to-speech engine to generate realistic speech audio for the
character. VoiceVox must be running as a separate service for this method to function. If VoiceVox is not available, the
method will silently fail without throwing an error.

## Requirements

- **VoiceVox Server**: Must be running and accessible
- **No error on failure**: If VoiceVox is unavailable, the method succeeds but no speech occurs

## Examples

### Basic Text-to-Speech

```typescript
import {Vrm} from '@homunculus/sdk';

const character = await Vrm.spawn('characters::speaker.vrm');

// Simple speech
await character.speakOnVoiceVox('Hello! Nice to meet you.');

// Longer speech
await character.speakOnVoiceVox(
    'Welcome to our application. I am your virtual assistant, ready to help you with any questions or tasks you might have.'
);
```

## Common Use Cases

### Character Dialogue

Make characters speak conversational text with appropriate voice selection.

### System Announcements

Use text-to-speech for important notifications and system messages.

### Educational Content

Provide spoken explanations and tutorials with subtitle support.

### Interactive Storytelling

Create immersive narrative experiences with multiple voices and characters.

### Accessibility Features

Convert text content to audio for users with visual impairments.

## Related Documentation

- **[VRM Character Management](index.md)** - Overall character system
- **[Vrm.setState()](setState.md)** - Coordinating speech with character states
- **[Webview System](../../webview/index.md)** - Creating custom subtitle interfaces
- **[Mod Asset System](../mods/index.md)** - Managing font and audio assets
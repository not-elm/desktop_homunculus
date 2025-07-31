# Voicevox Speaker

Gets the current VoiceVox speaker ID used for text-to-speech synthesis.
VoiceVox speakers represent different voice characters with unique tones, styles, and personalities that can be used to
give your VRM characters distinct voices.

## Parameters

- `options`: Optional configuration to scope the query to a specific VRM

### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to get VoiceVox speaker for specific character
}
```

## Returns

A promise that resolves to the current VoiceVox speaker ID as a number.

## Examples

### Basic Usage

```typescript
// Get global VoiceVox speaker setting
const globalSpeaker = await gpt.voicevoxSpeaker();
console.log("Global VoiceVox speaker:", globalSpeaker);

// Get VoiceVox speaker for specific VRM character
const vrm = await Vrm.findByName("Assistant");
const vrmSpeaker = await gpt.voicevoxSpeaker({vrm: vrm.entity});
console.log("Assistant VoiceVox speaker:", vrmSpeaker);
```

## Related Functions

- [`gpt.saveVoicevoxSpeaker()`](./saveVoicevoxSpeaker.md) - Set VoiceVox speaker for character
- [`gpt.chat()`](./chat.md) - Use VoiceVox speaker in conversations
- [`vrm.speakOnVoiceVox()`](../vrm/speakOnVoiceVox.md) - Direct VRM speech synthesis

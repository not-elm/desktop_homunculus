# Chat with AI Model

Sends a chat message to the AI model and optionally makes a VRM speak the response.
This is the primary method for interactive conversations with AI-powered VRM characters.

## Parameters

- `userMessage`: The message to send to the AI model
- `options`: Optional configuration for chat behavior and VRM integration

### ChatOptions

```typescript
interface ChatOptions extends SpeakOnVoiceVoxOptions {
    vrm: number; // VRM entity ID that will respond to the chat
}
```

### SpeakOnVoiceVoxOptions

```typescript
interface SpeakOnVoiceVoxOptions {
    speaker?: number;           // VoiceVox speaker ID
    pause?: number;             // Pause duration in seconds between sentences
    waitForCompletion?: boolean; // Whether to wait for speech to complete
    subtitle?: SubtitleOptions; // Subtitle display configuration
}

interface SubtitleOptions {
    font?: string;        // Mod asset ID of the font to use
    fontSize?: number;    // Font size for subtitles
    color?: [number, number, number, number]; // RGBA color array [r, g, b, a]
}
```

### ChatResponse

```typescript
interface ChatResponse {
    message: string;   // Raw AI response text
    dialogue: string;  // Processed text suitable for speech synthesis
    emotion: string;   // Detected or assigned emotion
}
```

## Examples

### Basic Chat

```typescript
// Simple chat without VRM
const response = await gpt.chat("What's the weather like today?");
console.log("AI Response:", response.message);
console.log("Emotion:", response.emotion);
```

### VRM Character Chat

```typescript
// Chat with VRM character that speaks the response
const vrm = await Vrm.findByName("Assistant");

const response = await gpt.chat("Tell me a interesting fact!", {
    vrm: vrm.entity,
});

console.log("Character said:", response.dialogue);
```

### Advanced VRM Chat with Voice Options

```typescript
// Chat with custom voice settings and subtitles
const vrm = await Vrm.findByName("Guide");

const response = await gpt.chat("Welcome to Desktop Homunculus! Let me explain how this works.", {
    vrm: vrm.entity,
    speaker: 2,              // Use VoiceVox speaker ID 2
    pause: 0.5,              // Half-second pause between sentences
    waitForCompletion: true, // Wait for speech to finish
    subtitle: {
        font: "custom::arial.ttf",  // Custom font from mod assets
        fontSize: 24,               // Larger subtitle text
        color: [1.0, 1.0, 1.0, 0.9] // White text with slight transparency
    }
});

console.log("AI Response:", response.message);
console.log("Emotion detected:", response.emotion);
```

## Related Functions

- [`gpt.model()`](./model.md) - Get current AI model
- [`gpt.systemPrompt()`](./systemPrompt.md) - Get system prompt configuration
- [`gpt.useWebSearch()`](./useWebSearch.md) - Check web search settings
- [`vrm.speakOnVoiceVox()`](../vrm/Vrm/speakOnVoiceVox.md) - Direct VRM speech synthesis

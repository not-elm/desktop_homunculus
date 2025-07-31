# System Prompt

Controls system prompt used to configure AI behavior.
The system prompt defines the AI's personality, role, and behavioral guidelines that influence how it responds to user
messages.

## gpt.systemPrompt()

### Parameters

- `options`: Optional configuration to scope the query to a specific VRM

#### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to get system prompt for specific character
}
```

## Returns

A promise that resolves to the current system prompt as a string.

## gpt.saveSystemPrompt()

### Parameters

- `prompt`: The system prompt text to set
- `options`: Optional configuration to scope the setting to a specific VRM

#### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to set system prompt for specific character
}
```

## Examples

### Basic Usage

```typescript
// Get the global system prompt
const globalPrompt = await gpt.systemPrompt();
console.log("Global system prompt:", globalPrompt);

// Get system prompt for a specific VRM character
const vrm = await Vrm.findByName("Assistant");
const vrmPrompt = await gpt.systemPrompt({vrm: vrm.entity});
console.log("VRM system prompt:", vrmPrompt);
```

## Related Functions

- [`gpt.saveSystemPrompt()`](./saveSystemPrompt.md) - Set the system prompt
- [`gpt.chat()`](./chat.md) - Use the system prompt in conversations
- [`gpt.model()`](./model.md) - Check which model will use the system prompt

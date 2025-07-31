# GPT Model Management

Gets the current GPT model being used for AI interactions.
This function can retrieve either the global model setting or the model configured for a specific VRM character.

## Parameters

- `options`: Optional configuration to scope the query to a specific VRM

### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to get model for specific character
}
```

## Returns

A promise that resolves to the current model name as a string.

## Examples

### Basic Usage

```typescript
// Get the global model setting
const currentModel = await gpt.model();
console.log("Current global model:", currentModel);
// Output: "gpt-4"

// Get model for a specific VRM character
const vrm = await Vrm.findByName("Assistant");
const vrmModel = await gpt.model({vrm: vrm.entity});
console.log("VRM model:", vrmModel);
```

## Related Functions

- [`gpt.availableModels()`](./availableModels.md) - Get list of available models
- [`gpt.saveModel()`](./saveModel.md) - Change the active model
- [`gpt.chat()`](./chat.md) - Use the current model for conversations

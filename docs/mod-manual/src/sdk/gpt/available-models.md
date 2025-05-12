# Available Models

Fetches the list of available GPT models from the configured providers.
This function returns all AI models that can be used with the chat system, including models from different providers
like OpenAI, Anthropic, and others.

## Parameters

None.

## Returns

A promise that resolves to an array of model names available for use.

## Examples

### Basic Model Listing

```typescript
// Get all available models
const models = await gpt.availableModels();
console.log("Available models:", models);
// Output: ["gpt-3.5-turbo", "gpt-4", "gpt-4-turbo", "claude-3-sonnet", "claude-3-opus"]

// Check if a specific model is available
const hasGPT4 = models.includes("gpt-4");
console.log("GPT-4 available:", hasGPT4);
```

## Related Functions

- [`gpt.model()`](./model.md) - Get currently selected model
- [`gpt.saveModel()`](./saveModel.md) - Set the active model
- [`gpt.chat()`](./chat.md) - Use the selected model for chat
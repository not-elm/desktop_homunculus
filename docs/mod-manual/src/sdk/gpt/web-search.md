# Web Search

Controls the web search setting for GPT interactions.
When enabled, the AI can search the web for current information to enhance responses with up-to-date data.

> [!WARNING]
> To use this feature, the ChatGPT model must support web search.

## gpt.useWebSearch()

### Parameters

- `options`: Optional configuration to scope the query to a specific VRM

### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to get web search setting for specific character
}
```

### Returns

A promise that resolves to the current web search setting as a boolean.

## gpt.saveUseWebSearch()

### Parameters

- `model`: The model name to set (must be from the available models list)
- `options`: Optional configuration to scope the setting to a specific VRM

### Options

```typescript
interface Options {
    vrm?: number; // VRM entity ID to set model for specific character
}
```

## Examples

### Basic Usage

```typescript
// Check global web search setting
const globalWebSearch = await gpt.useWebSearch();
console.log("Global web search enabled:", globalWebSearch);

// Check web search setting for specific VRM character
const vrm = await Vrm.findByName("NewsBot");
await gpt.saveUseWebSearch(true, {vrm: vrm.entity});
const vrmWebSearch = await gpt.useWebSearch({vrm: vrm.entity});
console.log("NewsBot web search enabled:", vrmWebSearch);
```

## Related Functions

- [`gpt.saveUseWebSearch()`](./saveUseWebSearch.md) - Enable/disable web search
- [`gpt.chat()`](./chat.md) - Use web search in conversations
- [`gpt.systemPrompt()`](./systemPrompt.md) - Configure how characters use web search information

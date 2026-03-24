/** A single user message yielded by an input adapter. */
export interface SDKUserMessage {
  type: "user";
  message: { role: "user"; content: string };
}

/** Contract for input adapters that feed user messages into the Claude Agent SDK. */
export interface InputAdapter {
  createAsyncGenerator(signal: AbortSignal): AsyncGenerator<SDKUserMessage>;
}

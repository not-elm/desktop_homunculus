/** Message shape expected by the Claude Agent SDK's streaming prompt input. */
export interface SDKUserMessage {
  type: "user";
  message: { role: "user"; content: string };
}

/**
 * Adapter that bridges an input source (PTT keyboard events or always-on STT)
 * into an AsyncGenerator of SDKUserMessage for the Claude Agent SDK.
 */
export interface InputAdapter {
  createAsyncGenerator(): AsyncGenerator<SDKUserMessage>;
  close(): void;
}

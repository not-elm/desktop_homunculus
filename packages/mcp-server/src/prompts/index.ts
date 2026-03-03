import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { registerDeveloperAssistantPrompt } from "./developer-assistant.js";
import { registerCharacterInteractionPrompt } from "./character-interaction.js";
import { registerModCommandHelperPrompt } from "./mod-command-helper.js";

export function registerPrompts(server: McpServer): void {
  registerDeveloperAssistantPrompt(server);
  registerCharacterInteractionPrompt(server);
  registerModCommandHelperPrompt(server);
}

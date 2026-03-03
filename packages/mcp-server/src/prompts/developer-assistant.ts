import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

export function registerDeveloperAssistantPrompt(server: McpServer): void {
  server.prompt(
    "developer-assistant",
    "Generate appropriate character reactions for development events",
    { event: z.string().describe("Development event: build-success, build-failure, test-pass, test-fail, git-push, git-commit, deploy") },
    (args) => ({
      messages: [{
        role: "user",
        content: {
          type: "text",
          text: `A development event occurred: "${args.event}". Use the play_reaction tool to make the desktop character react appropriately. Choose the best reaction from: happy, sad, confused, error, success, thinking, surprised, neutral. For success events use "success" or "happy". For failures use "error" or "sad". For uncertain outcomes use "thinking" or "confused".`,
        },
      }],
    }),
  );
}

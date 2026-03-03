import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

export function registerCharacterInteractionPrompt(server: McpServer): void {
  server.prompt(
    "character-interaction",
    "Have a natural interaction with the desktop character",
    {
      message: z.string().describe("What to say or do"),
      mood: z.string().optional().describe("Desired mood: happy, playful, serious, encouraging"),
    },
    (args) => ({
      messages: [{
        role: "user",
        content: {
          type: "text",
          text: `Interact with the desktop character. Message: "${args.message}". ${args.mood ? `Mood: ${args.mood}.` : ""} First use get_character_snapshot to check the current state, then use play_reaction for the appropriate expression, and optionally speak_message if the character should say something aloud.`,
        },
      }],
    }),
  );
}

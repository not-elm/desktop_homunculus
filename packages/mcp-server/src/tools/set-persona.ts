import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSetPersona(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "set_persona",
    "Set the active character's personality profile. This affects how the character is perceived in AI conversations.",
    {
      profile: z.string().describe("Character profile/background description (e.g. 'A cheerful assistant who loves coding')"),
      personality: z.string().optional().describe("Personality traits in natural language (e.g. 'Friendly, curious, and slightly sarcastic')"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        const vrm = new Vrm(entity);
        await vrm.setPersona({
          profile: args.profile,
          personality: args.personality ?? null,
          ocean: {},
          metadata: {},
        });
        return {
          content: [{
            type: "text" as const,
            text: `Set persona for character (entity ${entity}): "${args.profile}"`,
          }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

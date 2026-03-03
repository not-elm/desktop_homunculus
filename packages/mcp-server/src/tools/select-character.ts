import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSelectCharacter(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "select_character",
    "Switch the active character by name. All subsequent tools will target this character. Use get_character_snapshot to see available characters.",
    {
      name: z.string().describe("Character name to select"),
    },
    async (args) => {
      try {
        const vrm = await Vrm.findByName(args.name);
        state.activeCharacterEntity = vrm.entity;
        return {
          content: [{ type: "text" as const, text: `Active character switched to "${args.name}" (entity ${vrm.entity})` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

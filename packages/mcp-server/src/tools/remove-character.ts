import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerRemoveCharacter(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "remove_character",
    "Remove a VRM character from the desktop. If no name is given, removes the active character.",
    {
      name: z.string().optional().describe("Name of the character to remove. If omitted, removes the active character."),
    },
    async (args) => {
      try {
        let vrm: Vrm;
        if (args.name) {
          vrm = await Vrm.findByName(args.name);
        } else {
          const entity = await state.resolveCharacter();
          vrm = new Vrm(entity);
        }
        const entity = vrm.entity;
        await vrm.despawn();
        if (state.activeCharacterEntity === entity) {
          state.activeCharacterEntity = null;
        }
        return {
          content: [{ type: "text" as const, text: `Character (entity ${entity}) has been removed from the desktop.` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

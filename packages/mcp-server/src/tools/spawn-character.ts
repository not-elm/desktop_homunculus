import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm, entities } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSpawnCharacter(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "spawn_character",
    "Spawn a new VRM character on the desktop. Use the homunculus://assets resource to discover available VRM model assets. Returns the entity ID and name.",
    {
      asset: z.string().describe("VRM model asset ID (e.g. 'vrm:elmer')"),
      name: z.string().optional().describe("Display name for the character"),
      persona_profile: z.string().optional().describe("Character personality/background description"),
      x: z.number().optional().describe("Initial viewport X position (pixels)"),
      y: z.number().optional().describe("Initial viewport Y position (pixels)"),
    },
    async (args) => {
      try {
        const persona = args.persona_profile
          ? { profile: args.persona_profile, ocean: {}, metadata: {} }
          : undefined;

        const vrm = await Vrm.spawn(args.asset, { persona });

        if (args.x !== undefined && args.y !== undefined) {
          await entities.move(vrm.entity, { type: "viewport", position: [args.x, args.y] });
        }

        state.activeCharacterEntity = vrm.entity;

        const characterName = args.name ?? await entities.name(vrm.entity);
        return {
          content: [
            {
              type: "text" as const,
              text: `Spawned character "${characterName}" (entity ${vrm.entity})`,
            },
          ],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { entities } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerMoveCharacter(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "move_character",
    "Move the active character to a screen position. Coordinates are in viewport pixels (0,0 = top-left of primary monitor).",
    {
      x: z.number().describe("Viewport X coordinate (pixels)"),
      y: z.number().describe("Viewport Y coordinate (pixels)"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        await entities.move(entity, { type: "viewport", position: [args.x, args.y] });
        return {
          content: [{ type: "text" as const, text: `Character moved to (${args.x}, ${args.y})` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

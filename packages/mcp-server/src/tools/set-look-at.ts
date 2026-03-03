import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSetLookAt(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "set_look_at",
    'Control where the active character looks. Use "cursor" to follow the mouse cursor, or "none" to disable look-at (character looks forward).',
    {
      mode: z.enum(["cursor", "none"]).describe('Look-at mode: "cursor" to follow mouse, "none" to disable'),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        const vrm = new Vrm(entity);
        if (args.mode === "none") {
          await vrm.unlook();
          return {
            content: [{ type: "text" as const, text: "Look-at disabled. Character is now looking forward." }],
          };
        } else {
          await vrm.lookAtCursor();
          return {
            content: [{ type: "text" as const, text: "Character is now following the mouse cursor." }],
          };
        }
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

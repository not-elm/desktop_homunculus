import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { audio } from "@hmcs/sdk";
import { handleApiError } from "./shared.js";

export function registerControlBgm(server: McpServer): void {
  server.tool(
    "control_bgm",
    "Control background music playback. Actions: play (requires asset), stop, pause, resume, status.",
    {
      action: z.enum(["play", "stop", "pause", "resume", "status"]).describe("BGM action"),
      asset: z.string().optional().describe("MOD asset ID for 'play' action"),
      volume: z.number().min(0).max(1).optional().describe("Volume level"),
    },
    async (args) => {
      try {
        switch (args.action) {
          case "play":
            if (!args.asset) {
              return { content: [{ type: "text" as const, text: "Asset ID required for 'play' action" }], isError: true };
            }
            await audio.bgm.play(args.asset, { volume: args.volume });
            return { content: [{ type: "text" as const, text: `Playing BGM: ${args.asset}` }] };
          case "stop":
            await audio.bgm.stop();
            return { content: [{ type: "text" as const, text: "BGM stopped" }] };
          case "pause":
            await audio.bgm.pause();
            return { content: [{ type: "text" as const, text: "BGM paused" }] };
          case "resume":
            await audio.bgm.resume();
            return { content: [{ type: "text" as const, text: "BGM resumed" }] };
          case "status": {
            const status = await audio.bgm.status();
            return { content: [{ type: "text" as const, text: JSON.stringify(status, null, 2) }] };
          }
          default: {
            const _exhaustive: never = args.action;
            return { content: [{ type: "text" as const, text: `Unknown action: ${_exhaustive}` }], isError: true as const };
          }
        }
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

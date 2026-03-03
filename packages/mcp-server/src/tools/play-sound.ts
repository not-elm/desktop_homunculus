import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { audio } from "@hmcs/sdk";
import { handleApiError } from "./shared.js";

export function registerPlaySound(server: McpServer): void {
  server.tool(
    "play_sound",
    "Play a sound effect. Use a MOD asset ID (e.g., 'se:open') or a preset name.",
    {
      sound: z.string().describe("Sound asset ID or preset name"),
      volume: z.number().min(0).max(1).default(0.8).describe("Volume level (0.0-1.0)"),
    },
    async (args) => {
      try {
        await audio.se.play(args.sound, { volume: args.volume });
        return {
          content: [{ type: "text" as const, text: `Played sound: ${args.sound}` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

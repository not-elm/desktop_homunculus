import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm, audio, HomunculusApiError } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { REACTION_PRESETS } from "../presets/reactions.js";
import { handleApiError } from "./shared.js";

const REACTION_NAMES = Object.keys(REACTION_PRESETS);

export function registerPlayReaction(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "play_reaction",
    `Play a named reaction on the desktop character. The character will change expression and optionally play a sound effect. Available reactions: ${REACTION_NAMES.join(", ")}.`,
    {
      reaction: z.enum(REACTION_NAMES as [string, ...string[]]).describe("The reaction to play"),
      message: z.string().optional().describe("Optional text to display near the character"),
    },
    async (args) => {
      try {
        const preset = REACTION_PRESETS[args.reaction];
        if (!preset) {
          return {
            content: [{ type: "text" as const, text: `Unknown reaction: ${args.reaction}. Available: ${REACTION_NAMES.join(", ")}` }],
            isError: true,
          };
        }
        const entity = await state.resolveCharacter();
        const vrm = new Vrm(entity);
        const tasks: Promise<void>[] = [];
        const warnings: string[] = [];

        if (Object.keys(preset.expressions).length > 0) {
          tasks.push(vrm.modifyExpressions(preset.expressions));
        } else {
          tasks.push(vrm.clearExpressions());
        }

        const ignore404 = (label: string) => (err: unknown) => {
          if (err instanceof HomunculusApiError && err.statusCode === 404) return;
          warnings.push(`${label} failed: ${err instanceof Error ? err.message : String(err)}`);
        };

        if (preset.vrma) {
          tasks.push(
            vrm.playVrma({ asset: preset.vrma }).catch(ignore404("VRMA")),
          );
        }

        if (preset.se) {
          tasks.push(
            audio.se.play(preset.se, { volume: 0.8 }).catch(ignore404("Sound effect")),
          );
        }

        await Promise.all(tasks);

        let text = `Played reaction "${args.reaction}" on character (entity ${entity}).`;
        if (args.message) text += ` Message: ${args.message}`;
        if (warnings.length > 0) text += ` (warnings: ${warnings.join("; ")})`;
        return {
          content: [{ type: "text" as const, text }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

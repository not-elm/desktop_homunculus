import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { VrmaRepeat } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerPlayAnimation(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "play_animation",
    "Play a VRMA animation on the active character. Use the homunculus://assets resource to discover available VRMA animations.",
    {
      asset: z.string().describe("VRMA animation asset ID (e.g. 'vrma:idle-maid')"),
      repeat: z.string().optional().default("never").describe('Repeat mode: "never" (play once), "forever" (loop), or a number like "3" (play N times)'),
      transition_secs: z.number().optional().default(0.3).describe("Crossfade transition duration in seconds"),
      wait: z.boolean().optional().default(false).describe("If true, wait for the animation to complete before returning"),
      reset_spring_bones: z.boolean().optional().default(false).describe("If true, resets SpringBone velocities to prevent bouncing during animation transitions"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        const vrm = new Vrm(entity);

        let parsedRepeat: VrmaRepeat;
        if (args.repeat === "never") {
          parsedRepeat = { type: "never" };
        } else if (args.repeat === "forever") {
          parsedRepeat = { type: "forever" };
        } else {
          const count = Number(args.repeat);
          if (Number.isInteger(count) && count > 0) {
            parsedRepeat = { type: "count", count };
          } else {
            parsedRepeat = { type: "never" };
          }
        }

        await vrm.playVrma({
          asset: args.asset,
          repeat: parsedRepeat,
          transitionSecs: args.transition_secs,
          waitForCompletion: args.wait,
          resetSpringBones: args.reset_spring_bones,
        });

        const repeatDesc = parsedRepeat.type === "forever" ? " (looping)" : parsedRepeat.type === "count" ? ` (repeating ${parsedRepeat.count} times)` : "";
        const waitDesc = args.wait ? " (waited for completion)" : "";
        return {
          content: [{
            type: "text" as const,
            text: `Playing animation "${args.asset}" on character (entity ${entity})${repeatDesc}${waitDesc}`,
          }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

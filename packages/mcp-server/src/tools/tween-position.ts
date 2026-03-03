import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { entities } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

const easingFunctions = [
  "linear",
  "quadraticIn", "quadraticOut", "quadraticInOut",
  "cubicIn", "cubicOut", "cubicInOut",
  "quarticIn", "quarticOut", "quarticInOut",
  "quinticIn", "quinticOut", "quinticInOut",
  "sineIn", "sineOut", "sineInOut",
  "circularIn", "circularOut", "circularInOut",
  "exponentialIn", "exponentialOut", "exponentialInOut",
  "elasticIn", "elasticOut", "elasticInOut",
  "backIn", "backOut", "backInOut",
  "bounceIn", "bounceOut", "bounceInOut",
  "smoothStepIn", "smoothStepOut", "smoothStep",
  "smootherStepIn", "smootherStepOut", "smootherStep"
] as const;

export function registerTweenPosition(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "tween_position",
    "Smoothly animate an entity's position to a target value over time. Use this for smooth character movement and animations.",
    {
      targetX: z.number().describe("Target X coordinate in world space"),
      targetY: z.number().describe("Target Y coordinate in world space"),
      targetZ: z.number().describe("Target Z coordinate in world space"),
      durationMs: z.number().min(1).describe("Animation duration in milliseconds"),
      easing: z.enum(easingFunctions).optional().describe("Easing function for the animation (default: linear)"),
      wait: z.boolean().optional().describe("Whether to wait for animation to complete before returning (default: false)"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();

        await entities.tweenPosition(entity, {
          target: [args.targetX, args.targetY, args.targetZ],
          durationMs: args.durationMs,
          easing: args.easing,
          wait: args.wait,
        });

        const waitMsg = args.wait ? " (waited for completion)" : "";
        return {
          content: [{
            type: "text" as const,
            text: `Position tween started: moving to (${args.targetX}, ${args.targetY}, ${args.targetZ}) over ${args.durationMs}ms${waitMsg}`,
          }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

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

export function registerTweenRotation(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "tween_rotation",
    "Smoothly animate an entity's rotation to a target value over time. Rotation is specified as a quaternion (x, y, z, w).",
    {
      targetX: z.number().describe("Target quaternion X component"),
      targetY: z.number().describe("Target quaternion Y component"),
      targetZ: z.number().describe("Target quaternion Z component"),
      targetW: z.number().describe("Target quaternion W component"),
      durationMs: z.number().min(1).describe("Animation duration in milliseconds"),
      easing: z.enum(easingFunctions).optional().describe("Easing function for the animation (default: linear)"),
      wait: z.boolean().optional().describe("Whether to wait for animation to complete before returning (default: false)"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();

        await entities.tweenRotation(entity, {
          target: [args.targetX, args.targetY, args.targetZ, args.targetW],
          durationMs: args.durationMs,
          easing: args.easing,
          wait: args.wait,
        });

        const waitMsg = args.wait ? " (waited for completion)" : "";
        return {
          content: [{
            type: "text" as const,
            text: `Rotation tween started: rotating to quaternion (${args.targetX}, ${args.targetY}, ${args.targetZ}, ${args.targetW}) over ${args.durationMs}ms${waitMsg}`,
          }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

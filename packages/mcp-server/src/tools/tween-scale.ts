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

export function registerTweenScale(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "tween_scale",
    "Smoothly animate an entity's scale to a target value over time. Use this for grow/shrink effects and size animations.",
    {
      targetX: z.number().min(0).describe("Target scale X factor (1.0 = normal size)"),
      targetY: z.number().min(0).describe("Target scale Y factor (1.0 = normal size)"),
      targetZ: z.number().min(0).describe("Target scale Z factor (1.0 = normal size)"),
      durationMs: z.number().min(1).describe("Animation duration in milliseconds"),
      easing: z.enum(easingFunctions).optional().describe("Easing function for the animation (default: linear)"),
      wait: z.boolean().optional().describe("Whether to wait for animation to complete before returning (default: false)"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();

        await entities.tweenScale(entity, {
          target: [args.targetX, args.targetY, args.targetZ],
          durationMs: args.durationMs,
          easing: args.easing,
          wait: args.wait,
        });

        const waitMsg = args.wait ? " (waited for completion)" : "";
        return {
          content: [{
            type: "text" as const,
            text: `Scale tween started: scaling to (${args.targetX}, ${args.targetY}, ${args.targetZ}) over ${args.durationMs}ms${waitMsg}`,
          }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

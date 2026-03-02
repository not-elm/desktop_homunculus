import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Vrm } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerSetExpression(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "set_expression",
    'Set facial expression weights on the active character. Common expressions: happy, sad, angry, surprised, relaxed, neutral, aa, ih, ou, ee, oh, blink. Weights are 0.0-1.0. Modes: "modify" (default, partial update), "set" (replace all), "clear" (reset to animation control). For preset reactions, use play_reaction instead.',
    {
      expressions: z.record(z.string(), z.number().min(0).max(1)).optional().describe("Expression name to weight map. Required unless mode is \"clear\"."),
      mode: z.enum(["set", "modify", "clear"]).optional().default("modify").describe("Operation mode"),
    },
    async (args) => {
      try {
        const entity = await state.resolveCharacter();
        const vrm = new Vrm(entity);

        switch (args.mode) {
          case "clear": {
            await vrm.clearExpressions();
            return {
              content: [{ type: "text" as const, text: "Cleared all expression overrides." }],
            };
          }
          case "set": {
            if (!args.expressions || Object.keys(args.expressions).length === 0) {
              return {
                content: [{ type: "text" as const, text: "expressions parameter is required and must be non-empty for \"set\" mode." }],
                isError: true,
              };
            }
            await vrm.setExpressions(args.expressions);
            return {
              content: [{ type: "text" as const, text: `Set expressions: ${JSON.stringify(args.expressions)}` }],
            };
          }
          case "modify":
          default: {
            if (!args.expressions || Object.keys(args.expressions).length === 0) {
              return {
                content: [{ type: "text" as const, text: "expressions parameter is required and must be non-empty for \"modify\" mode." }],
                isError: true,
              };
            }
            await vrm.modifyExpressions(args.expressions);
            return {
              content: [{ type: "text" as const, text: `Modified expressions: ${JSON.stringify(args.expressions)}` }],
            };
          }
        }
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

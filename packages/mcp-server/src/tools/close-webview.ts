import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Webview } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerCloseWebview(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "close_webview",
    "Close a webview panel. If no entity ID is given, closes the most recently opened webview. Use all=true to close all webviews.",
    {
      entity: z.number().optional().describe("Entity ID of the webview to close. If omitted, closes the most recently opened."),
      all: z.boolean().optional().default(false).describe("Close all open webviews (default: false)"),
    },
    async (args) => {
      try {
        if (args.all) {
          const webviews = await Webview.list();
          if (webviews.length === 0) {
            return {
              content: [{ type: "text" as const, text: "No webviews are open." }],
            };
          }
          const results = await Promise.allSettled(webviews.map((w) => new Webview(w.entity).close()));
          state.clearWebviews();
          const failures = results.filter((r) => r.status === "rejected");
          if (failures.length > 0) {
            return {
              content: [{ type: "text" as const, text: `Closed ${webviews.length - failures.length} webview(s), ${failures.length} failed.` }],
              isError: failures.length === webviews.length,
            };
          }
          return {
            content: [{ type: "text" as const, text: `Closed ${webviews.length} webview(s).` }],
          };
        }

        let targetEntity: number;
        if (args.entity !== undefined) {
          targetEntity = args.entity;
        } else {
          const last = state.lastWebview();
          if (last === null) {
            return {
              content: [{ type: "text" as const, text: "No webviews tracked." }],
              isError: true,
            };
          }
          targetEntity = last;
        }

        const webview = new Webview(targetEntity);
        await webview.close();
        state.untrackWebview(targetEntity);
        return {
          content: [{ type: "text" as const, text: `Closed webview (entity ${targetEntity}).` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}

import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";
import { Webview, webviewSource } from "@hmcs/sdk";
import type { HomunculusMcpState } from "../state.js";
import { handleApiError } from "./shared.js";

export function registerNavigateWebview(server: McpServer, state: HomunculusMcpState): void {
  server.tool(
    "navigate_webview",
    "Navigate an existing webview to new HTML content. Use this to update a webview's content without closing and reopening it. If no entity is specified, navigates the most recently opened webview.",
    {
      entity: z.number().optional().describe("Entity ID of the webview to navigate. If omitted, navigates the most recently opened."),
      html: z.string().describe("New inline HTML content to display"),
    },
    async (args) => {
      try {
        let targetEntity: number;
        if (args.entity !== undefined) {
          targetEntity = args.entity;
        } else {
          const last = state.lastWebview();
          if (last === null) {
            return {
              content: [{ type: "text" as const, text: "No webviews tracked. Open a webview first with open_webview." }],
              isError: true,
            };
          }
          targetEntity = last;
        }

        const webview = new Webview(targetEntity);
        await webview.navigate(webviewSource.html(args.html));

        return {
          content: [{ type: "text" as const, text: `Navigated webview (entity ${targetEntity}) to new content.` }],
        };
      } catch (error) {
        return handleApiError(error);
      }
    },
  );
}
